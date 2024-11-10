use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from, ReadHelper};

// Consts for SampleDescriptionBox
const SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION: std::ops::RangeFrom<usize> = 12..;

// Constants for fixed sizes
const SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT_SIZE: usize = 4; // 4 bytes for sample_count

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
        let sample_description = buffer[SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION].to_vec();

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

// Implementing ReadHelper trait for SampleDescriptionBox
impl ReadHelper for SampleDescriptionBox {
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size()
    }

    fn total_size(&self) -> usize {
        let header_size = self.header.total_size(); // Size of the BoxHeader
        let sample_count_size = SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT_SIZE; // Size of sample_count (4 bytes)
        let sample_description_size = self.sample_description.len(); // Variable length

        // Total size is the sum of fixed sizes + variable size
        header_size + sample_count_size + sample_description_size
    }
}
