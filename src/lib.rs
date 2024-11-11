mod boxes;
mod mp4;
mod shared_consts;

use boxes::header::BoxHeader;
use mp4::MP4;

pub struct MP4Parser {}

impl MP4Parser {
    pub fn from_buf(buffer: &[u8]) -> MP4 {
        let mut seek: usize = 0;
        let mut mp4 = mp4::MP4::default();

        // Loop until the end of the buffer is reached
        while seek < buffer.len() {
            println!("---- seek at {} -----", seek);

            // Ensure there is enough data for a BoxHeader
            if buffer.len() < seek + 8 {
                println!("Insufficient data for BoxHeader at position {}", seek);
                break;
            }

            // Parse the header for the current box
            let current_data = &buffer[seek..];
            let header = BoxHeader::from_buffer(current_data);
            println!("detected header: {}", header.box_type());

            // Match the type of box to parse
            match header.box_type().as_str() {
                boxes::ftyp::HEADER_FTYP => {
                    let ftyp = boxes::ftyp::Ftyp::from_buffer(current_data);
                    seek += ftyp.header().size();
                    mp4.ftyp = Some(ftyp);
                }
                boxes::moov::HEADER_MOOV => {
                    let movie = boxes::moov::MovieBox::from_buffer(current_data);
                    println!("{:?}", movie);
                    seek += movie.header().size();
                    println!("{} size of moov", movie.header().size());
                    mp4.moov = Some(movie);
                }
                _ => {
                    // Skip unrecognized boxes by advancing `seek` by the box's full size
                    seek += header.size();
                    println!("Skipped unknown box type at position {}", seek);
                }
            }
        }

        mp4
    }
}
