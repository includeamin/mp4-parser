use super::header::BoxHeader;

// Consts for SampleDescriptionBox
const SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug, Clone)]
pub struct SampleDescriptionBox {
    header: BoxHeader,           // Size and type at offset 0–7
    sample_count: u32,           // 4 bytes at offset 8–11
    sample_description: Vec<u8>, // Variable length after offset 12
}

impl SampleDescriptionBox {
    // `from_buffer` takes a seek position and buffer and returns a fully constructed SampleDescriptionBox
    pub fn from_buffer(buffer: &[u8]) -> Self {
        // Read the header at the beginning
        let header = BoxHeader::from_buffer(buffer);

        // Read the sample_count field (4 bytes)
        let sample_count = u32::from_be_bytes(
            buffer[SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT]
                .try_into()
                .expect("Failed to read sample_count"),
        );

        // Read the sample_description field (variable-length)
        let sample_description =
            buffer[SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION.start..header.size()].to_vec();

        SampleDescriptionBox {
            header,
            sample_count,
            sample_description,
        }
    }

    // Getter for sample_count
    pub fn get_sample_count(&self) -> u32 {
        self.sample_count
    }

    // Getter for sample_description
    pub fn get_sample_description(&self) -> &[u8] {
        &self.sample_description
    }

    // Getter for header
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test the from_buffer method and getters
    #[test]
    fn test_sample_description_box_from_buffer() {
        // Prepare a mock buffer
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleDescriptionBox (size = 32 bytes, type = "stsd")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b's', b'd', // type = "stsd"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00, 0x03, // sample_count = 3
            // sample_description (variable-length)
            0x01, 0x02, 0x03, 0x04, 0x05, // sample_description = [1, 2, 3, 4, 5]
        ];

        // Construct the SampleDescriptionBox from the buffer
        let sample_description_box = SampleDescriptionBox::from_buffer(&mock_buffer);

        // Test header extraction
        assert_eq!(sample_description_box.get_header().size(), 32);
        assert_eq!(sample_description_box.get_header().box_type(), "stsd");

        // Test sample_count extraction
        assert_eq!(sample_description_box.get_sample_count(), 3);

        // Test sample_description extraction
        let expected_description: Vec<u8> = vec![0x01, 0x02, 0x03, 0x04, 0x05];
        assert_eq!(
            sample_description_box.get_sample_description(),
            expected_description.as_slice()
        );
    }

    // Test buffer with no sample_description data
    #[test]
    fn test_sample_description_box_no_description() {
        // Prepare a buffer with sample_description as empty (edge case)
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleDescriptionBox (size = 32 bytes, type = "stsd")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b's', b'd', // type = "stsd"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00,
            0x01, // sample_count = 1

                  // Empty sample_description (no bytes)
        ];

        // Construct the SampleDescriptionBox from the buffer
        let sample_description_box = SampleDescriptionBox::from_buffer(&mock_buffer);

        // Test sample_count extraction
        assert_eq!(sample_description_box.get_sample_count(), 1);

        // Test sample_description extraction (should be empty)
        let expected_description: Vec<u8> = vec![];
        assert_eq!(
            sample_description_box.get_sample_description(),
            expected_description.as_slice()
        );
    }

    // Test with a larger buffer for sample_description data
    #[test]
    fn test_sample_description_box_large_description() {
        // Prepare a buffer with a large sample_description
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleDescriptionBox (size = 32 bytes, type = "stsd")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b's', b'd', // type = "stsd"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00, 0x02, // sample_count = 2
            // sample_description (large)
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF, // 16 bytes
        ];

        // Construct the SampleDescriptionBox from the buffer
        let sample_description_box = SampleDescriptionBox::from_buffer(&mock_buffer);

        // Test sample_count extraction
        assert_eq!(sample_description_box.get_sample_count(), 2);

        // Test sample_description extraction
        let expected_description: Vec<u8> = vec![
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE,
            0xFF,
        ];
        assert_eq!(
            sample_description_box.get_sample_description(),
            expected_description.as_slice()
        );
    }
}
