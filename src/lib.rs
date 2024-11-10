mod boxes;
mod mp4;
mod shared_consts;
mod utils;

use boxes::header::BoxHeader;
use mp4::MP4;

pub struct MP4Parser {}

impl MP4Parser {
    pub fn from_buf(buffer: &[u8]) -> MP4 {
        let mut seek: usize = 0;
        let mut mp4 = mp4::MP4::default();
        loop {
            println!("---- seek at {} -----", seek);
            let header = BoxHeader::from_buffer(buffer);
            println!("detected header: {}", header.box_type());
            match header.box_type().as_str() {
                boxes::ftyp::HEADER_FTYP => {
                    let ftyp = boxes::ftyp::Ftyp::from_buffer(buffer);
                    seek += ftyp.header().size();
                    mp4.ftyp = Some(ftyp)
                }
                boxes::moov::HEADER_MOOV => {
                    let movie = boxes::moov::MovieBox::from_buffer(buffer);
                    println!("{:?}", movie);
                    seek += movie.header().size();
                    println!("{}", seek);
                    mp4.moov = Some(movie);
                }
                _ => {
                    break;
                }
            }
        }

        mp4
    }
}
