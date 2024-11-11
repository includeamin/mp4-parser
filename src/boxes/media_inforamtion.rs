use super::header::BoxHeader;
use super::sample_table::SampleTableBox;

#[derive(Debug, Clone)]
pub struct MediaInformationBox {
    header: BoxHeader, // Size and type at offset 0–7
    stbl: SampleTableBox,
}

impl MediaInformationBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let stbl = SampleTableBox::from_buffer(&buffer[8..]);

        MediaInformationBox { header, stbl }
    }

    // Getter for the header
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    // Getter for the stbl (Sample Table Box)
    pub fn get_stbl(&self) -> &SampleTableBox {
        &self.stbl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the `from_buffer` function for `MediaInformationBox`
    #[test]
    fn test_media_information_box_from_buffer() {
        // Create a mock buffer that represents a MediaInformationBox
        let mock_buffer: &[u8] = &[
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

        // Call the `from_buffer` method to parse the buffer
        let media_information_box = MediaInformationBox::from_buffer(mock_buffer);

        // Test BoxHeader parsing (size, type)
        assert_eq!(media_information_box.header().size(), 171); // Size should be 20 bytes
        assert_eq!(media_information_box.header().box_type(), "minf"); // Type should be "minf"

        // Test SampleTableBox parsing
        assert_eq!(media_information_box.get_stbl().get_header().size(), 136); // Size of the SampleTableBox should be 6 bytes
        assert_eq!(
            media_information_box.get_stbl().get_header().box_type(),
            "stbl"
        ); // Type should be "stbl"
    }

    // Test invalid buffer scenario for `MediaInformationBox` where the buffer is too short
    #[test]
    fn test_invalid_media_information_box_buffer() {
        // Create a mock buffer that is too short to contain both BoxHeader and SampleTableBox
        let mock_buffer: &[u8] = &[
            // Only 8 bytes, which is just enough for the BoxHeader
            0x00, 0x00, 0x00, 0x08, // Box size
            0x6D, 0x69, 0x6E, 0x66, // Box type "minf"
        ];

        // Try to parse the buffer (this should likely panic or return an error, depending on your implementation)
        let result = std::panic::catch_unwind(|| {
            MediaInformationBox::from_buffer(mock_buffer);
        });

        // Assert that the code panicked due to an invalid buffer (this depends on your error handling)
        assert!(result.is_err());
    }

    // Test getter methods for `BoxHeader` and `SampleTableBox`
    #[test]
    fn test_media_information_box_getters() {
        let mock_buffer: &[u8] = &[
            // Mock BoxHeader data (8 bytes)
            0x00, 0x00, 0x00, 0x14, // Box size
            0x6D, 0x69, 0x6E, 0x66, // Box type "minf"
            // SampleTableBox mock data
            0x00, 0x00, 0x00, 0x06, // SampleTableBox size
            0x73, 0x74, 0x62, 0x6C, 0x00, 0x00, // "stbl" type + mock data
        ];

        let media_information_box = MediaInformationBox::from_buffer(mock_buffer);

        // Check the header values using getters
        assert_eq!(media_information_box.header().size(), 0x14); // Size should be 20 bytes
        assert_eq!(media_information_box.header().box_type(), "minf"); // Box type should be "minf"

        // Check the SampleTableBox values using getter
        assert_eq!(media_information_box.get_stbl().get_header().size(), 0x06); // Size of SampleTableBox should be 6 bytes
        assert_eq!(
            media_information_box.get_stbl().get_header().box_type(),
            "stbl"
        ); // Box type should be "stbl"
    }
}
