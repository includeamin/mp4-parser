use super::header::BoxHeader;
use super::{movie_header::MovieHeaderBox, track::TrackBox};
use crate::utils::get_range_from;

pub const HEADER_MOOV: &str = "moov";
const MOVIE_BOX_TRAKS: std::ops::RangeFrom<usize> = 12..; // Variable-length, depends on the box structure

#[derive(Debug)]
pub struct MovieBox {
    header: BoxHeader, // Size and type at offset 0–7
    mvhd: MovieHeaderBox,
    traks: Vec<TrackBox>, // List of Track Boxes
}

impl MovieBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let mvhd = MovieHeaderBox::from_buffer(seek, buffer);

        // Extract the traks (Track Boxes)
        let mut traks = Vec::new();
        let traks_range = get_range_from(seek, MOVIE_BOX_TRAKS);
        let mut trak_offset = traks_range.start;
        // let end_at = header.size

        println!("{:?}", header);
        // loop {
        //     traks.push(TrackBox::from_buffer(trak_offset, buffer));
        //     trak_offset +=traks.last().unwrap().header.size as usize;
        // }
        // while trak_offset < traks_range.end {
        //     traks.push(TrackBox::from_buffer(trak_offset, buffer));
        //     trak_offset += traks.last().unwrap().header.size as usize; // Move to the next track
        // }

        MovieBox {
            header,
            mvhd,
            traks,
        }
    }
}
