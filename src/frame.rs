use smallvec::SmallVec;
use std::io::ErrorKind;
use std::io::Read;

use crate::header::{parse_frame_header, FrameHeader};
use crate::Mp3Error;

pub struct Frame {
    header: FrameHeader,
    // TODO find out which number would be useful here.
    data: SmallVec<[u8; 256]>,
}

impl Frame {
    pub fn header(&self) -> &FrameHeader {
        &self.header
    }
}

pub struct FrameReader<R: Read> {
    rdr: R,
}

impl<R: Read> FrameReader<R> {
    pub fn new(rdr: R) -> Self {
        FrameReader { rdr }
    }
    pub fn next_frame(&mut self) -> Result<Option<Frame>, Mp3Error> {
        let mut header_data = [0; 4];
        match self.rdr.read_exact(&mut header_data) {
            Ok(_) => (),
            Err(e) => {
                // If we reached the end of the file, let's return None,
                // otherwise pass on the error.
                if e.kind() == ErrorKind::UnexpectedEof {
                    return Ok(None);
                } else {
                    return Err(Mp3Error::from(e));
                }
            }
        }
        let header = parse_frame_header(&header_data)?;
        let len = header.frame_size();
        if let Some(len) = len {
            let mut data = smallvec![0; len - 4];
            self.rdr.read_exact(&mut data)?;
            Ok(Some(Frame { header, data }))
        } else {
            unimplemented!("Free format not implemented")
        }
    }
}
