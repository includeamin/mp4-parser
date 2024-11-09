use super::header::BoxHeader;

use super::{meida::MediaBox, track_header::TrackHeaderBox};

#[derive(Debug)]
pub struct TrackBox {
    header: BoxHeader, // Size and type at offset 0–7
    tkhd: TrackHeaderBox,
    mdia: MediaBox,
}

impl TrackBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let tkhd = TrackHeaderBox::from_buffer(seek, buffer);
        let mdia = MediaBox::from_buffer(seek, buffer);

        TrackBox { header, tkhd, mdia }
    }
}
