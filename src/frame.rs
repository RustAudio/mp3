use std::io::ErrorKind;
use std::io::Read;

use header::{parse_frame_header, Bitrate, FrameHeader, Layer};
use Mp3Error;

impl FrameHeader {
    pub fn frame_size(&self) -> Option<usize> {
        match self.bitrate {
            Bitrate::FreeFormat => None,

            Bitrate::Indexed(bitrate) => {
                // The number of bytes a slot occupies
                // This is described in sections 2.1 and 2.4.2.1 of ISO/IEC 11172-3
                let slot_size = match self.layer {
                    Layer::LayerI => 4_usize,
                    _ => 1_usize,
                };

                // Now compute the number of slots.
                // This is described in section 2.4.3.1 of ISO/IEC 11172-3

                let multiplier = match self.layer {
                    Layer::LayerI => 12,
                    _ => 144000,
                };

                let mut slot_count =
                    (multiplier * (bitrate as usize)) / (self.sampling_rate as usize);

                if self.padding {
                    slot_count += 1;
                }

                Some(slot_count * slot_size)
            }
        }
    }
}

pub struct Frame {
    header: FrameHeader,
    data: Vec<u8>,
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
            let mut data = vec![0; len - 4];
            self.rdr.read_exact(&mut data)?;
            Ok(Some(Frame { header, data }))
        } else {
            unimplemented!("Free format not implemented")
        }
    }
}
