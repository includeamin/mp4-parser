mod boxes;
mod mp4;
mod shared_consts;
mod utils;

use boxes::header::BoxHeader;
use mp4::MP4;
use utils::ReadHelper;

pub struct MP4Parser {}

impl MP4Parser {
    pub fn from_buf(buffer: &[u8]) -> MP4 {
        let mut seek: usize = 0;
        let mut mp4 = mp4::MP4::default();
        loop {
            println!("---- seek at {} -----", seek);
            let header = BoxHeader::from_buffer(seek, buffer);
            println!("detected header: {}", header.get_box_type());
            match header.get_box_type().as_str() {
                boxes::ftyp::HEADER_FTYP => {
                    let ftyp = boxes::ftyp::Ftyp::from_buffer(seek, buffer);
                    seek = seek + boxes::ftyp::Ftyp::get_end_range(seek);
                    mp4.ftyp = Some(ftyp)
                }
                boxes::movie::HEADER_MOOV => {
                    let movie = boxes::movie::MovieBox::from_buffer(seek, buffer);
                    println!("{:?}", movie);
                    mp4.moov = Some(movie);
                    break;
                }
                _ => {}
            }
        }

        return mp4;
    }
}
