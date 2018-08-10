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
    match str::from_utf8(&data[..3]) {
        Ok("ID3") => {
            let id3_len = (data[6] as usize) * 128 * 128 * 128
                + (data[7] as usize) * 128 * 128
                + (data[8] as usize) * 128
                + (data[9] as usize);

            return Ok(&data[10 + id3_len..]);
        }
        Ok(_) => {
            let len = data.len();

            // Check if it is ID3v1
            match str::from_utf8(&data[len - ID3V1_LEN..len - ID3V1_LEN + 3]) {
                Ok("TAG") => return Ok(&data[..len - ID3V1_LEN]),
                _ => return Err(Mp3Error::ID3Error),
            }
        }
        Err(e) => Err(Mp3Error::Utf8Error(e)),
    }
}

pub fn frame_size(header: &FrameHeader) -> usize {
    (144000_usize * (header.bitrate as usize)) / (header.sampling_rate as usize)
        + match (header.padding, &header.layer) {
            (true, &Layer::LayerI) => 4_usize,
            (true, _) => 1_usize,
            _ => 0_usize,
        }
}
