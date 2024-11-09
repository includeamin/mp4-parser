use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from};

// Consts for SampleDescriptionBox
const SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug)]
pub struct SampleDescriptionBox {
    header: BoxHeader,           // Size and type at offset 0–7
    sample_count: u32,           // 4 bytes at offset 8–11
    sample_description: Vec<u8>, // Variable length after offset 12
}

impl SampleDescriptionBox {
    // `from_buffer` takes a seek position and buffer and returns a fully constructed SampleDescriptionBox
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        // Read the header at the beginning
        let header = BoxHeader::from_buffer(seek, buffer);

        // Read the sample_count field (4 bytes)
        let sample_count = u32::from_be_bytes(
            buffer[get_range(seek, SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT)]
                .try_into()
                .expect("Failed to read sample_count"),
        );

        // Read the sample_description field (variable-length)
        let sample_description =
            buffer[get_range_from(seek, SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION)].to_vec();

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