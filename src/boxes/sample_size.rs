use super::header::BoxHeader;

const SAMPLE_SIZE_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_SIZE_BOX_SAMPLE_SIZES: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug, Clone)]
pub struct SampleSizeBox {
    header: BoxHeader,      // Size and type at offset 0–7
    sample_count: u32,      // 4 bytes at offset 8–11
    sample_sizes: Vec<u32>, // Variable length after offset 12
}

impl SampleSizeBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        // Parse the header at the beginning of the box
        let header = BoxHeader::from_buffer(buffer);

        // Parse the sample_count field (4 bytes)
        let sample_count = u32::from_be_bytes(
            buffer[SAMPLE_SIZE_BOX_SAMPLE_COUNT]
                .try_into()
                .expect("Failed to read sample_count"),
        );

        // Parse the sample_sizes field (variable-length field, each sample size is 4 bytes)
        let sample_sizes = buffer[SAMPLE_SIZE_BOX_SAMPLE_SIZES.start..header.size()]
            .chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().expect("Failed to read sample size")))
            .collect::<Vec<_>>();

        SampleSizeBox {
            header,
            sample_count,
            sample_sizes,
        }
    }

    // Getter for sample_count
    pub fn get_sample_count(&self) -> u32 {
        self.sample_count
    }

    // Getter for sample_sizes
    pub fn get_sample_sizes(&self) -> &[u32] {
        &self.sample_sizes
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
    fn test_sample_size_box_from_buffer() {
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleSizeBox (size = 24 bytes, type = "stsz")
            0x00, 0x00, 0x00,
            0x18, // size = 24 (total size: header(8 bytes) + sample_count(4 bytes) + sample_sizes(12 bytes))
            b's', b't', b'z', b's', // type = "stsz"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00, 0x03, // sample_count = 3
            // sample_sizes (variable-length)
            0x00, 0x00, 0x00, 0x01, // sample_size = 1
            0x00, 0x00, 0x00, 0x02, // sample_size = 2
            0x00, 0x00, 0x00, 0x03, // sample_size = 3
        ];

        // Construct the SampleSizeBox from the buffer
        let sample_size_box = SampleSizeBox::from_buffer(&mock_buffer);

        // Test header extraction
        assert_eq!(sample_size_box.get_header().size(), 24);
        assert_eq!(sample_size_box.get_header().box_type(), "stzs");

        // Test sample_count extraction
        assert_eq!(sample_size_box.get_sample_count(), 3);

        // Test sample_sizes extraction
        let expected_sample_sizes = [1, 2, 3];
        assert_eq!(
            sample_size_box.get_sample_sizes(),
            &expected_sample_sizes[..]
        );
    }

    // Test buffer with no sample_sizes data (edge case)
    #[test]
    fn test_sample_size_box_no_sizes() {
        // Prepare a buffer with sample_sizes as empty (edge case)
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleSizeBox (size = 12 bytes, type = "stsz")
            0x00, 0x00, 0x00, 0x0C, // size = 12 (header(8 bytes) + sample_count(4 bytes))
            b's', b't', b'z', b's', // type = "stsz"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00,
            0x01, // sample_count = 1

                  // Empty sample_sizes (no bytes)
        ];

        // Construct the SampleSizeBox from the buffer
        let sample_size_box = SampleSizeBox::from_buffer(&mock_buffer);

        // Test sample_count extraction
        assert_eq!(sample_size_box.get_sample_count(), 1);

        // Test sample_sizes extraction (should be empty)
        let expected_sample_sizes: Vec<u32> = vec![];
        assert_eq!(
            sample_size_box.get_sample_sizes(),
            expected_sample_sizes.as_slice()
        );
    }

    // Test with a larger buffer for sample_sizes data
    #[test]
    fn test_sample_size_box_large_sizes() {
        // Prepare a buffer with sample_sizes field of size 3 (matching sample_count)
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleSizeBox (size = 20 bytes, type = "stsz")
            0x00, 0x00, 0x00,
            0x18, // size = 20 (header(8 bytes) + sample_count(4 bytes) + sample_sizes(12 bytes))
            b's', b't', b'z', b's', // type = "stsz"
            // sample_count (4 bytes)
            0x00, 0x00, 0x00, 0x03, // sample_count = 3
            // sample_sizes (variable-length)
            0x00, 0x00, 0x00, 0x01, // sample_size = 1
            0x00, 0x00, 0x00, 0x02, // sample_size = 2
            0x00, 0x00, 0x00, 0x03, // sample_size = 3
        ];

        // Construct the SampleSizeBox from the buffer
        let sample_size_box = SampleSizeBox::from_buffer(&mock_buffer);

        // Test sample_count extraction
        assert_eq!(sample_size_box.get_sample_count(), 3);

        // Test sample_sizes extraction
        let expected_sample_sizes = [1, 2, 3];
        assert_eq!(
            sample_size_box.get_sample_sizes(),
            &expected_sample_sizes[..]
        );
    }
}
