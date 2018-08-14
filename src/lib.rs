#![forbid(unsafe_code)]

#[macro_use]
extern crate smallvec;

pub mod frame;
pub mod header;
pub mod tables;

use std::io;
use std::str;

pub static ID3V1_LEN: usize = 128;

#[derive(Debug)]
pub enum Mp3Error {
    // Unable to trim ID3 tag
    ID3Error,
    // Incorrect Header
    HeaderError,
    IoError(io::Error),
    Utf8Error(str::Utf8Error),
}

impl From<io::Error> for Mp3Error {
    fn from(e: io::Error) -> Mp3Error {
        Mp3Error::IoError(e)
    }
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
