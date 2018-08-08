use std::str;

#[derive(Debug)]
pub enum Mp3Error {
    ID3Error, // Unable to trim ID3 tag
    HeaderError, // Incorrect Header
    Utf8Error(str::Utf8Error),
}

impl From<str::Utf8Error> for Mp3Error {
    fn from(e: str::Utf8Error) -> Mp3Error {
        Mp3Error::Utf8Error(e)
    }
}
