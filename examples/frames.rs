extern crate mp3;

use mp3::frame::FrameReader;

use std::fs::File;
use std::io::Read;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut f = File::open(path).unwrap();
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let slice = mp3::strip_id3(&buf).unwrap();
    let mut frame_reader = FrameReader::new(slice);
    while let Some(v) = frame_reader.next_frame().unwrap() {
        println!("{:?}", v.header());
    }
}
