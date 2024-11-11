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
        // Parse the header (first 8 bytes)
        let header = BoxHeader::from_buffer(buffer);

        // Parse the MediaHeaderBox (mdhd), which starts after the header (8 bytes)
        let mdhd = MediaHeaderBox::from_buffer(&buffer[8..]);

        // Parse the HandlerBox (hdlr), which starts after the mdhd box
        // We use the size of the mdhd box (8 bytes header + mdhd size)
        println!("mdhd.header().size() {}", mdhd.header().size());
        let hdlr = HandlerBox::from_buffer(&buffer[8 + mdhd.header().size()..]);

        // Parse the MediaInformationBox (minf), which starts after the hdlr box
        // We use the size of the hdlr box (8 bytes header + hdlr size)
        let minf = MediaInformationBox::from_buffer(&buffer[8 + mdhd.header().size() + hdlr.header().size()..]);

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

    // Mock data representing a MediaBox buffer (simplified)
    const MOCK_MEDIA_BOX_BUFFER: &[u8] = &[
        // BoxHeader - size: 60 bytes (header + mdhd + hdlr + minf)
        0x49, 0x02, 0x00, 0x00, // Size of box (60 bytes total)
        0x6D, 0x64, 0x69, 0x61, // Type ("mdia")
        // MediaHeaderBox (mdhd) - size: 20 bytes
        // Mock BoxHeader data (8 bytes)
        0x00, 0x00, 0x00, 0x14, // Box size
        0x6D, 0x64, 0x68, 0x64, // Box type "mdhd"
        // Version and flags (1 byte + 3 bytes)
        0x01, // Version
        0x00, 0x00, 0x00, // Flags
        // Creation time (4 bytes)
        0x00, 0x00, 0x00, 0x01, // Creation time
        // Modification time (4 bytes)
        0x00, 0x00, 0x00, 0x02, // Modification time
        // Timescale (4 bytes)
        0x00, 0x00, 0x00, 0x03, // Timescale
        // Duration (4 bytes)
        0x00, 0x00, 0x00, 0x04, // Duration
        // Language (2 bytes, ISO-639-2 format)
        0x6E, 0x61, // "eng" (English)
        // HandlerBox (hdlr) - size: 32 bytes
        0x00, 0x00, 0x00, 0x29, // size = 41 bytes (adjusted)
        0x68, 0x64, 0x6C, 0x72, // type = "hdlr"
        0x01, 0x00, 0x00, 0x01, // Mock version and flags (4 bytes total)
        0x76, 0x69, 0x64, 0x65, // flags
        0x00, 0x00, 0x00, 0x00, // handler_type = "vide"
        0x00, 0x00, 0x00, 0x00, // reserved
        0x00, 0x00, 0x00, 0x00, // null-terminator
        0x54, 0x65, 0x73, 0x74, // null-terminator
        0x20, 0x48, 0x61, 0x6E, // null-terminator
        0x64, 0x6C, 0x65, 0x72, 0x00, // null-terminator
        // MediaInformationBox (minf) - size: 48 bytes
        // Mock BoxHeader data (8 bytes)
        0x00, 0x00, 0x00, 0xAB, // 171 Box size
        0x6D, 0x69, 0x6E, 0x66, // Box type "minf"
        // SampleTableBox (stbl) - Outer box
        0x00, 0x00, 0x00, 0x88, // Box size (163 bytes total) - u32
        0x73, 0x74, 0x62, 0x6C, // Box type ("stbl") - 4 bytes
        // SampleDescriptionBox (stsd) - Sub-box
        0x00, 0x00, 0x00, 0x10, // Size field (16 bytes) - u32
        0x73, 0x74, 0x73, 0x64, // Type field ("stsd") - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Version and flags - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Entry count (1) - u32
        // TimeToSampleBox (stts) - Sub-box
        0x00, 0x00, 0x00, 0x24, // Box size (36 bytes) - u32
        0x73, 0x74, 0x74, 0x73, // Box type ("stts") - 4 bytes
        0x00, 0x00, 0x00, 0x04, // Entry count (4 entries) - u32
        // Entry 1
        0x00, 0x00, 0x00, 0x0A, // Sample count (10) - u32
        0x00, 0x00, 0x00, 0x64, // Sample delta (100) - u32
        // Entry 2
        0x00, 0x00, 0x00, 0x05, // Sample count (5) - u32
        0x00, 0x00, 0x00, 0xC8, // Sample delta (200) - u32
        // Entry 3
        0x00, 0x00, 0x00, 0x08, // Sample count (8) - u32
        0x00, 0x00, 0x01, 0x2C, // Sample delta (300) - u32
        // SampleToChunkBox (stsc) - Sub-box
        0x00, 0x00, 0x00, 0x24, // Box size (36 bytes) - u32
        0x73, 0x74, 0x73, 0x63, // Type field ("stsc") - 4 bytes
        0x00, 0x00, 0x00, 0x03, // Entry count (3) - u32
        // Entry 1
        0x00, 0x00, 0x00, 0x01, // First chunk (1) - u32
        0x00, 0x00, 0x00, 0x64, // Samples per chunk (100) - u32
        // Entry 2
        0x00, 0x00, 0x00, 0x02, // Second chunk (2) - u32
        0x00, 0x00, 0x00, 0xC8, // Samples per chunk (200) - u32
        // Entry 3
        0x00, 0x00, 0x00, 0x03, // Third chunk (3) - u32
        0x00, 0x00, 0x01, 0x2C, // Samples per chunk (300) - u32
        // SampleSizeBox (stsz) - Sub-box
        0x00, 0x00, 0x00, 0x18, // Box size (24 bytes) - u32
        0x73, 0x74, 0x73, 0x7A, // Box type ("stsz") - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Sample size entry count (1) - u32
        0x00, 0x00, 0x04, 0x00, // Sample size (1024) - u32
        0x00, 0x00, 0x00, 0x05, // Sample count (5) - u32
        // Sample sizes
        0x00, 0x00, 0x04, 0x00, // Sample size (1024) - u32
        // ChunkOffsetBox (stco) - Sub-box
        0x00, 0x00, 0x00, 0x10, // Size field (16 bytes) - u32
        0x73, 0x74, 0x63, 0x6F, // Type field ("stco") - 4 bytes
        0x00, 0x00, 0x00, 0x02, // Entry count (2) - u32
        // Chunk offsets
        0x00, 0x00, 0x00, 0x20, // Offset for chunk 1 (32 bytes) - u32
        0x00, 0x00, 0x00, 0x40, // Offset for chunk 2 (64 bytes) - u32
    ];

    #[test]
    fn test_media_box_creation() {
        // Use the mock buffer to create a MediaBox instance
        let buffer = MOCK_MEDIA_BOX_BUFFER;
        let media_box = MediaBox::from_buffer(buffer);

        // Assert that the header is parsed correctly
        assert_eq!(media_box.get_header().size(), 36);
        assert_eq!(media_box.get_header().box_type(), "mdia");

        // Assert that the MediaHeaderBox (mdhd) is parsed correctly
        assert_eq!(media_box.get_mdhd().header().size(), 16);
        assert_eq!(media_box.get_mdhd().header().box_type(), "mdhd");

        // Assert that the HandlerBox (hdlr) is parsed correctly
        assert_eq!(media_box.get_hdlr().header().size(), 32);
        assert_eq!(media_box.get_hdlr().header().box_type(), "hdlr");

        // Assert that the MediaInformationBox (minf) is parsed correctly
        assert_eq!(media_box.get_minf().header().size(), 48);
        assert_eq!(media_box.get_minf().header().box_type(), "minf");
    }

    #[test]
    fn test_media_box_getters() {
        let buffer = MOCK_MEDIA_BOX_BUFFER;
        let media_box = MediaBox::from_buffer(buffer);

        // Test the getters
        assert_eq!(media_box.get_header().size(), 36);
        assert_eq!(media_box.get_mdhd().header().box_type(), "mdhd");
        assert_eq!(media_box.get_hdlr().header().box_type(), "hdlr");
        assert_eq!(media_box.get_minf().header().box_type(), "minf");
    }
}
