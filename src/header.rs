use tables::*;
use Mp3Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Version {
    Mpeg2_5,
    Reserved,
    Mpeg2,
    Mpeg1,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Layer {
    LayerI,
    LayerII,
    LayerIII,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Bitrate {
    Indexed(u16),
    FreeFormat,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    Stereo,
    JointStereo,
    DualChannel,
    Mono,
}

/*
|------|----------------|------------------------------|
|      | Layer I and II |          Layer III           |
|------|----------------|------------------|-----------|
| bits |  Layer I & II  | Intensity Stereo | MS Stereo |
|------|----------------|------------------|-----------|
|  00  | bands 4 to 31  |        Off       |    Off    |
|  01  | bands 8 to 31  |        On        |    Off    |
|  10  | bands 12 to 31 |        Off       |    On     |
|  11  | bands 16 to 31 |        On        |    On     |
|------|----------------|------------------|-----------|
*/
#[derive(Debug, Clone, PartialEq)]
pub enum ModeExtension {
    Bands(u8),
    Stereo(bool, bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Emphasis {
    None,
    Ms50_15, // 50/15 ms
    Reserved,
    CcittJ17, // CCITT J.17
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameHeader {
    // MPEG Audio version
    version: Version,
    // MPEG layer
    pub layer: Layer,
    // Indicates that frame is protected by CRC (16 bit CRC follows header).
    protection: bool,
    // Bit rate
    pub bitrate: Bitrate,
    // Sampling rate
    pub sampling_rate: u16,
    // Indicates that frame is padded with one extra slot (32 bits for Layer I, 8 bits for others).
    pub padding: bool,
    // Bit for application-specific triggers.
    private: bool,
    // Channel Mode
    mode: Mode,
    // Only used in Joint stereo mode to join informations that are of no use for stereo effect.
    mode_extension: ModeExtension,
    // Indicates that audio is copyrighted.
    copyright: bool,
    // Indicates that the frame is located on its original media.
    original: bool,
    // Tells the decoder that the file must be de-emphasized. Rarely used.
    emphasis: Emphasis,
}

pub fn parse_frame_header(data: &[u8]) -> Result<FrameHeader, Mp3Error> {
    let header = &data[..4];

    // Sync word check
    if (header[0] != 0xff_u8) || (header[1] & 0xe0_u8 != 0xe0_u8) {
        return Err(Mp3Error::HeaderError);
    }

    let version = match header[1] & 0x18_u8 {
        0 => Version::Mpeg2_5,
        0x08_u8 => Version::Reserved,
        0x10_u8 => Version::Mpeg2,
        0x18_u8 => Version::Mpeg1,
        _ => return Err(Mp3Error::HeaderError),
    };

    let layer = match header[1] & 0x06_u8 {
        0x02_u8 => Layer::LayerIII,
        0x04_u8 => Layer::LayerII,
        0x06_u8 => Layer::LayerI,
        _ => return Err(Mp3Error::HeaderError),
    };

    let protection = header[1] & 0x01_u8 != 0;

    let bitrate_index = (header[2] >> 4) as usize;

    if bitrate_index == 15 {
        return Err(Mp3Error::HeaderError);
    }

    let bitrate = if bitrate_index != 0 {
        let rate = match (&version, &layer) {
            (&Version::Mpeg1, &Layer::LayerI) => BITRATE_INDEX[0][bitrate_index],
            (&Version::Mpeg1, &Layer::LayerII) => BITRATE_INDEX[1][bitrate_index],
            (&Version::Mpeg1, &Layer::LayerIII) => BITRATE_INDEX[2][bitrate_index],
            (_, &Layer::LayerI) => BITRATE_INDEX[3][bitrate_index],
            (_, _) => BITRATE_INDEX[4][bitrate_index],
        };

        Bitrate::Indexed(rate)
    } else {
        Bitrate::FreeFormat
    };

    let sampling_rate_index = ((header[2] & 0x0c_u8) >> 2) as usize;

    if sampling_rate_index == 3 {
        return Err(Mp3Error::HeaderError);
    }

    let sampling_rate = match &version {
        &Version::Mpeg1 => SAMPLING_RATE[0][sampling_rate_index],
        &Version::Mpeg2 => SAMPLING_RATE[1][sampling_rate_index],
        &Version::Mpeg2_5 => SAMPLING_RATE[2][sampling_rate_index],
        _ => return Err(Mp3Error::HeaderError),
    };

    let padding = header[2] & 0x02_u8 != 0;

    let private = header[2] & 0x01_u8 != 0;

    let mode = match header[3] & 0xc0_u8 {
        0 => Mode::Stereo,
        0x40_u8 => Mode::JointStereo,
        0x80_u8 => Mode::DualChannel,
        0xc0_u8 => Mode::Mono,
        _ => unreachable!(),
    };

    let mode_extension_bits = header[3] & 0x30_u8;

    let mode_extension = match &layer {
        &Layer::LayerIII => match mode_extension_bits {
            0 => ModeExtension::Stereo(false, false),
            0x10_u8 => ModeExtension::Stereo(true, false),
            0x20_u8 => ModeExtension::Stereo(false, true),
            0x30_u8 => ModeExtension::Stereo(true, true),
            _ => unreachable!(),
        },
        &Layer::LayerI | &Layer::LayerII => match mode_extension_bits {
            0 => ModeExtension::Bands(4),
            0x10_u8 => ModeExtension::Bands(8),
            0x20_u8 => ModeExtension::Bands(12),
            0x30_u8 => ModeExtension::Bands(16),
            _ => unreachable!(),
        },
    };

    let copyright = header[3] & 0x08_u8 != 0;

    let original = header[3] & 0x04_u8 != 0;

    let emphasis = match header[3] & 0x03_u8 {
        0 => Emphasis::None,
        0x01_u8 => Emphasis::Ms50_15,
        0x02_u8 => Emphasis::Reserved,
        0x03_u8 => Emphasis::CcittJ17,
        _ => unreachable!(),
    };

    Ok(FrameHeader {
        version,
        layer,
        protection,
        bitrate,
        sampling_rate,
        padding,
        private,
        mode,
        mode_extension,
        copyright,
        original,
        emphasis,
    })
}
