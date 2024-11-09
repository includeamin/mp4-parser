use super::header::BoxHeader;
use super::{
    handler::HandlerBox, media_header::MediaHeaderBox, media_inforamtion::MediaInformationBox,
};

#[derive(Debug)]
pub struct MediaBox {
    header: BoxHeader,         // Size and type at offset 0–7
    mdhd: MediaHeaderBox,      // Media header box
    hdlr: HandlerBox,          // Handler box
    minf: MediaInformationBox, // Media information box
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

    // Getter for the header
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }

    // Getter for the MediaHeaderBox (mdhd)
    pub fn get_mdhd(&self) -> &MediaHeaderBox {
        &self.mdhd
    }

    // Getter for the HandlerBox (hdlr)
    pub fn get_hdlr(&self) -> &HandlerBox {
        &self.hdlr
    }

    // Getter for the MediaInformationBox (minf)
    pub fn get_minf(&self) -> &MediaInformationBox {
        &self.minf
    }
}
