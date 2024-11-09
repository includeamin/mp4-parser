mod ftyp;
mod header;
mod moov;
mod shared_consts;
mod utils;

use ftyp::FtypBox;
use header::BoxHeader;

struct MP4Parser {}

impl MP4Parser {
    fn parser(buffer: &[u8]) {
        let mut seek: usize = 0;
        loop {
            println!("---- seek at {} -----", seek);
            let header = BoxHeader::from_buffer(seek, buffer);
            println!("detected header: {}", header.name);
            match header.name.as_str() {
                ftyp::HEADER_FTYP => {
                    let ftype = FtypBox::from_buffer(seek, buffer);
                    println!("{:?}", ftype);
                    seek = seek + FtypBox::get_end_range(seek);
                }
                moov::HEADER_MOOV => {}
                _ => {}
            }
        }
    }
}
