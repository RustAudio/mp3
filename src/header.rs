use error::Mp3Error;
use tables::*;

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
pub enum Protection {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Private {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Padding {
    Yes,
    No,
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
pub enum Copyright {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Original {
    Yes,
    No,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Emphasis {
    None,
    Ms50_15,  // 50/15 ms
    Reserved,
    CcittJ17,  // CCITT J.17
}

#[derive(Debug, Clone, PartialEq)]
pub struct FrameHeader {
    version: Version,
    layer: Layer,
    protection: Protection,
    bitrate: u16, // kbps
    sampling_rate: u16, // Hz
    padding: Padding,
    private: Private,
    mode: Mode,
    mode_extension: ModeExtension,
    copyright: Copyright,
    original: Original,
    emphasis: Emphasis,
}


pub fn frame_header(data: &[u8]) -> Result<FrameHeader, Mp3Error> {
    let header = &data[..4];

    // Sync word check
    if (header[0] != 255 as u8) && (header[1] < 0b11100000u8) {
        return Err(Mp3Error::HeaderError)
    }

    let version = match header[1] & 0b00011000u8 {
        0b00000000u8 => Ok(Version::Mpeg2_5),
        0b00001000u8 => Ok(Version::Reserved),
        0b00010000u8 => Ok(Version::Mpeg2),
        0b00011000u8 => Ok(Version::Mpeg1),
        _            => Err(Mp3Error::HeaderError),
    }?;

    let layer = match header[1] & 0b00000110u8 {
        0b00000010u8 => Ok(Layer::LayerIII),
        0b00000100u8 => Ok(Layer::LayerII),
        0b00000110u8 => Ok(Layer::LayerI),
        _            => Err(Mp3Error::HeaderError),
    }?;

    let protection = match header[1] & 0b00000001u8 {
        0b00000001u8 => Protection::Yes,
        0u8          => Protection::No,
        _            => unreachable!(),
    };

    let bitrate_index = (header[2] >> 4) as usize;

    if bitrate_index == 0 || bitrate_index == 15 {
        return Err(Mp3Error::HeaderError);
    }

    let bitrate = match (&version, &layer) {
        (&Version::Mpeg1, &Layer::LayerI)   => BITRATE_INDEX[0][bitrate_index],
        (&Version::Mpeg1, &Layer::LayerII)  => BITRATE_INDEX[1][bitrate_index],
        (&Version::Mpeg1, &Layer::LayerIII) => BITRATE_INDEX[2][bitrate_index],
        (_, &Layer::LayerI)                 => BITRATE_INDEX[3][bitrate_index],
        (_, _)                              => BITRATE_INDEX[4][bitrate_index],
    };

    let sampling_rate_index = ((header[2] & 0b00001100u8) >> 2) as usize;

    if sampling_rate_index == 3 {
        return Err(Mp3Error::HeaderError);
    }

    let sampling_rate = match &version {
        &Version::Mpeg1   => Ok(SAMPLING_RATE[0][sampling_rate_index]),
        &Version::Mpeg2   => Ok(SAMPLING_RATE[1][sampling_rate_index]),
        &Version::Mpeg2_5 => Ok(SAMPLING_RATE[2][sampling_rate_index]),
        _                 => Err(Mp3Error::HeaderError),
    }?;

    let padding = match header[2] & 0b00000010u8 {
        0u8          => Padding::No,
        0b00000010u8 => Padding::Yes,
        _            => unreachable!(),
    };

    let private = match header[2] & 1u8 {
        0u8 => Private::No,
        1u8 => Private::Yes,
        _   => unreachable!(),
    };

    let mode = match header[3] & 0b11000000u8 {
        0u8        => Mode::Stereo,
        0b01000000 => Mode::JointStereo,
        0b10000000 => Mode::DualChannel,
        0b11000000 => Mode::Mono,
        _          => unreachable!(),
    };

    let mode_extension_bits = header[3] & 0b00110000u8;

    let mode_extension = match &layer {
        &Layer::LayerIII => match mode_extension_bits {
            0u8          => ModeExtension::Stereo(false, false),
            0b00010000u8 => ModeExtension::Stereo(true, false),
            0b00100000u8 => ModeExtension::Stereo(false, true),
            0b00110000u8 => ModeExtension::Stereo(true, true),
            _            => unreachable!(),
        },
        &Layer::LayerI | &Layer::LayerII => match mode_extension_bits {
            0u8          => ModeExtension::Bands(4),
            0b00010000u8 => ModeExtension::Bands(8),
            0b00100000u8 => ModeExtension::Bands(12),
            0b00110000u8 => ModeExtension::Bands(16),
            _            => unreachable!(),
        },
    };

    let copyright = match header[3] & 0b00001000u8 {
        0b00000000u8 => Copyright::No,
        0b00001000u8 => Copyright::Yes,
        _            => unreachable!(),
    };

    let original = match header[3] & 0b00000100u8 {
        0b00000000u8 => Original::No,
        0b00000100u8 => Original::Yes,
        _            => unreachable!(),
    };

    let emphasis = match header[3] & 0b00000011u8 {
        0b00000000u8 => Emphasis::None,
        0b00000001u8 => Emphasis::Ms50_15,
        0b00000010u8 => Emphasis::Reserved,
        0b00000011u8 => Emphasis::CcittJ17,
        _            => unreachable!(),
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

impl FrameHeader {
    pub fn bitrate(&self) -> u16 {
        self.bitrate
    }

    pub fn sampling_rate(&self) -> u16 {
        self.sampling_rate
    }

    pub fn padding(&self) -> Padding {
        self.padding.clone()
    }
}
