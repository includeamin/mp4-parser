use super::header::BoxHeader;
use super::{
    handler::HandlerBox, media_header::MediaHeaderBox, media_inforamtion::MediaInformationBox,
};

#[derive(Debug, Clone)]
pub struct MediaBox {
    header: BoxHeader,         // Size and type at offset 0–7
    mdhd: MediaHeaderBox,      // Media header box
    hdlr: HandlerBox,          // Handler box
    minf: MediaInformationBox, // Media information box
}

impl MediaBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let mdhd = MediaHeaderBox::from_buffer(buffer);
        let hdlr = HandlerBox::from_buffer(buffer);
        let minf = MediaInformationBox::from_buffer(buffer);

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

//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::header::BoxHeader;
//     use crate::handler::HandlerBox;
//     use crate::media_header::MediaHeaderBox;
//     use crate::media_inforamtion::MediaInformationBox;
//
//     #[test]
//     fn test_media_box_from_buffer() {
//         let buffer: &[u8] = &[
//             // Mock BoxHeader for MediaBox (8 bytes)
//             0x00, 0x00, 0x00, 0x58, // size = 88 bytes (0x58)
//             b'm', b'd', b'i', b'a',  // type = "mdia"
//             // Mock MediaHeaderBox (mdhd)
//             0x00, 0x00, 0x00, 0x20, // mdhd size
//             b'm', b'd', b'h', b'd',  // type = "mdhd"
//             // MediaHeaderBox content (e.g., version, flags, and other fields)
//             0x00,                    // version
//             0x00, 0x00, 0x03,        // flags
//             0x00, 0x00, 0x00, 0x02,  // other mdhd fields...
//
//             // Mock HandlerBox (hdlr)
//             0x00, 0x00, 0x00, 0x30, // hdlr size
//             b'h', b'd', b'l', b'r',  // type = "hdlr"
//             0x01,                    // version
//             0x00, 0x00, 0x01,        // flags
//             b'v', b'i', b'd', b'e',   // handler_type = "vide"
//             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, // reserved
//             b'T', b'e', b's', b't', b' ', b'H', b'a', b'n', b'd', b'l', b'e', b'r', 0, // name
//
//             // Mock MediaInformationBox (minf)
//             0x00, 0x00, 0x00, 0x10, // minf size
//             b'm', b'i', b'n', b'f',  // type = "minf"
//             // MediaInformationBox content
//             0x00, 0x00, 0x00, 0x00, // additional fields (example)
//         ];
//
//         let media_box = MediaBox::from_buffer(buffer);
//
//         // Test MediaBox header
//         assert_eq!(media_box.get_header().box_type, *b"mdia");
//         assert_eq!(media_box.get_header().size, 88);
//
//         // Test MediaHeaderBox (mdhd)
//         let mdhd = media_box.get_mdhd();
//         assert_eq!(mdhd.header.box_type, *b"mdhd");
//         assert_eq!(mdhd.header.size, 32);
//         assert_eq!(mdhd.version, 0);
//         assert_eq!(mdhd.flags, [0, 0, 3]);
//
//         // Test HandlerBox (hdlr)
//         let hdlr = media_box.get_hdlr();
//         assert_eq!(hdlr.header.box_type, *b"hdlr");
//         assert_eq!(hdlr.header.size, 48);
//         assert_eq!(hdlr.version, 1);
//         assert_eq!(hdlr.flags, [0, 0, 1]);
//         assert_eq!(hdlr.handler_type, *b"vide");
//         assert_eq!(hdlr.name(), "Test Handler");
//
//         // Test MediaInformationBox (minf)
//         let minf = media_box.get_minf();
//         assert_eq!(minf.header.box_type, *b"minf");
//         assert_eq!(minf.header.size, 16);
//
//         // Test total size and end range
//         assert_eq!(media_box.total_size(), 88);
//         assert_eq!(media_box.get_end_range(0), 88);
//     }
// }
