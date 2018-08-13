#![forbid(unsafe_code)]

pub mod header;
pub mod tables;

use header::*;
use std::str;

pub static ID3V1_LEN: usize = 128;

#[derive(Debug)]
pub enum Mp3Error {
    // Unable to trim ID3 tag
    ID3Error,
    // Incorrect Header
    HeaderError,
    Utf8Error(str::Utf8Error),
}

impl From<str::Utf8Error> for Mp3Error {
    fn from(e: str::Utf8Error) -> Mp3Error {
        Mp3Error::Utf8Error(e)
    }
}

// Trim ID3 tag from data and find first frame
pub fn trim_data(data: &[u8]) -> Result<&[u8], Mp3Error> {
    // Check if it is ID3v2
    match &data[..3] {
        b"ID3" => {
            let id3_len = (data[6] as usize) * 128 * 128 * 128
                + (data[7] as usize) * 128 * 128
                + (data[8] as usize) * 128
                + (data[9] as usize);

            return Ok(&data[10 + id3_len..]);
        }
        _ => {
            let start_of_tag = data.len() - ID3V1_LEN;

            // Check if it is ID3v1
            match &data[start_of_tag..start_of_tag + 3] {
                b"TAG" => return Ok(&data[..start_of_tag]),
                _ => return Err(Mp3Error::ID3Error),
            }
        }
    }
}

pub fn frame_size(header: &FrameHeader) -> Option<usize> {
    match header.bitrate {
        Bitrate::FreeFormat => None,

        Bitrate::Indexed(bitrate) => {
            let padding_size = match (header.padding, &header.layer) {
                (true, &Layer::LayerI) => 4_usize,
                (true, _) => 1_usize,
                _ => 0_usize,
            };

            let samples_per_frame = match header.layer {
                Layer::LayerI => 384_usize,
                _ => 1152_usize,
            };

            let byterate = bitrate as usize * 1000 / 8;

            let frame_size_times_sampling_rate = samples_per_frame * byterate;

            let frame_size = frame_size_times_sampling_rate / (header.sampling_rate as usize);

            Some(frame_size + padding_size)
        }
    }
}
