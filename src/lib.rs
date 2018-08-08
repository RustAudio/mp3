#![forbid(unsafe_code)]

pub mod error;
pub mod header;
pub mod tables;

use std::str;

use error::Mp3Error;
use header::*;

pub static ID3V1_LEN: usize = 128;

// Trim ID3 tag from data and find first frame
pub fn trim_data(data: &[u8]) -> Result<&[u8], Mp3Error> {
     // Check if it is ID3v2
    match str::from_utf8(&data[..3]) {
        Ok("ID3") => {
            let id3_len = (data[6] as usize) * 128 * 128 * 128
                        + (data[7] as usize) * 128 * 128
                        + (data[8] as usize) * 128
                        + (data[9] as usize);

            return Ok(&data[10 + id3_len ..]);
        },
        Ok(_) => {
            let len = data.len();

            // Check if it is ID3v1
            match str::from_utf8(&data[len - ID3V1_LEN .. len - ID3V1_LEN + 3]) {
                Ok("TAG") => {
                    return Ok(&data[..len - ID3V1_LEN])
                },
                _ => return Err(Mp3Error::ID3Error),
            }
        },
        Err(e) => Err(Mp3Error::Utf8Error(e)),
    }
}

pub fn frame_size(header: &FrameHeader) -> usize {
    (144000usize * (header.bitrate() as usize)) / (header.sampling_rate() as usize)
    + match header.padding() {
        Padding::Yes => 1usize,
        Padding::No  => 0usize,
    }
}
