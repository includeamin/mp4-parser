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
        let mdhd = MediaHeaderBox::from_buffer(&buffer[8..]);
        let hdlr = HandlerBox::from_buffer(&buffer[mdhd.header().size()..]);
        let minf = MediaInformationBox::from_buffer(&buffer[hdlr.header().size()..]);

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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_mock_buffer() -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // MediaBox Header (8 bytes): Size and type for MediaBox
        let media_box_size: i32 = 8 + 32 + 44 + 40; // Total size = 8 (header) + 32 (mdhd) + 44 (hdlr) + 40 (minf)
        buffer.extend_from_slice(&media_box_size.to_be_bytes()); // Size (4 bytes)
        buffer.extend_from_slice(b"mdia"); // Type (4 bytes)

        // BoxHeader for "mdhd" (size = 32 bytes, type = "mdhd")
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x20]); // Size = 32 bytes (8 for header + 24 for content)
        buffer.extend_from_slice(b"mdhd"); // Type = "mdhd"

        // Version and Flags (4 bytes total)
        buffer.push(0); // version
        buffer.extend_from_slice(&[0, 0, 0]); // flags

        // Timescale (4 bytes)
        buffer.extend_from_slice(&[0x00, 0x00, 0x03, 0xE8]); // timescale = 1000 (0x000003E8)

        // Duration (4 bytes)
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]); // duration = 0 (for simplicity)

        // Language (2 bytes)
        buffer.extend_from_slice(&[0x55, 0xC4]); // Language = "und" (undetermined)

        // Reserved (2 bytes)
        buffer.extend_from_slice(&[0x00, 0x00]); // reserved

        // Handler Box (hdlr) placeholder for simplicity
        // BoxHeader for "hdlr" (size = 44 bytes, type = "hdlr")
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x2C]); // Size = 44 bytes (8 for header + 36 for content)
        buffer.extend_from_slice(b"hdlr"); // Type = "hdlr"

        // Version and Flags (4 bytes total)
        buffer.push(1); // version
        buffer.extend_from_slice(&[0, 0, 1]); // flags

        // Handler Type ("vide" for video)
        buffer.extend_from_slice(b"vide"); // handler_type = "vide"

        // Reserved (12 bytes)
        buffer.extend_from_slice(&[0; 12]); // reserved

        // Handler Name (null-terminated string)
        buffer.extend_from_slice(b"Test Handler"); // name = "Test Handler"
        buffer.push(0); // null-terminator

        // Media Information Box (minf) placeholder
        // BoxHeader for "minf" (size = 40 bytes, type = "minf")
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x28]); // Size = 40 bytes (8 for header + 32 for content)
        buffer.extend_from_slice(b"minf"); // Type = "minf"

        // BoxHeader for "stts" (size = 32 bytes, type = "stts")
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x20]); // Size = 32 bytes (8 for header + 24 for content)
        buffer.extend_from_slice(b"stts"); // Type = "stts"

        // Version and Flags (4 bytes total)
        buffer.push(0); // version
        buffer.extend_from_slice(&[0, 0, 0]); // flags

        // Entry Count (4 bytes) - number of entries in the mapping (e.g., 1 entry for simplicity)
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]); // 1 entry

        // Sample-to-time mapping entry (Sample Count, Sample Delta)
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x01]); // Sample Count = 1
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x64]); // Sample Delta = 100 (0x64)

        // Return the complete buffer
        buffer
    }

    #[test]
    fn test_media_box_from_buffer() {
        // Create the valid mock buffer
        let buffer = create_valid_mock_buffer();

        // Parse the MediaBox from the buffer
        let media_box = MediaBox::from_buffer(&buffer);

        // Verify the header
        let header = media_box.get_header();
        assert_eq!(header.size(), 32 + 44 + 40); // Total size = 32 (mdhd) + 44 (hdlr) + 40 (minf)
        assert_eq!(header.box_type(), "mdhd"); // Type should be "mdhd"

        // Verify the MediaHeaderBox
        let mdhd = media_box.get_mdhd();
        assert_eq!(mdhd.get_timescale(), 1000); // timescale = 1000
        assert_eq!(mdhd.get_duration(), 0); // duration = 0
        assert_eq!(mdhd.get_language(), "und"); // language = "und"

        // Verify the HandlerBox
        let hdlr = media_box.get_hdlr();
        assert_eq!(hdlr.handler_type(), "vide"); // handler_type = "vide"
        assert_eq!(hdlr.name(), "Test Handler"); // handler_name = "Test Handler"

        // Verify the MediaInformationBox
        let minf = media_box.get_minf();
        assert_eq!(minf.header().box_type(), "minf"); // Type should be "minf"
    }
}
