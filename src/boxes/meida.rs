use super::header::BoxHeader;
use super::{
    handler::HandlerBox, media_header::MediaHeaderBox, media_inforamtion::MediaInformationBox,
};

#[derive(Debug)]
pub struct MediaBox {
    header: BoxHeader, // Size and type at offset 0–7
    mdhd: MediaHeaderBox,
    hdlr: HandlerBox,
    minf: MediaInformationBox,
}

impl MediaBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let mdhd = MediaHeaderBox::from_buffer(seek, buffer);
        let hdlr = HandlerBox::from_buffer(seek, buffer);
        let minf = MediaInformationBox::from_buffer(seek, buffer);

        MediaBox {
            header,
            mdhd,
            hdlr,
            minf,
        }
    }
}
