use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from, ReadHelper};

const SAMPLE_SIZE_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_SIZE_BOX_SAMPLE_SIZES: std::ops::RangeFrom<usize> = 12..;

// Constants for fixed sizes
const SAMPLE_SIZE_BOX_SAMPLE_COUNT_SIZE: usize = 4; // 4 bytes for sample_count
const SAMPLE_SIZE_BOX_SAMPLE_SIZE: usize = 4; // 4 bytes per sample size

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
        let sample_sizes = buffer[SAMPLE_SIZE_BOX_SAMPLE_SIZES]
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

// Implementing ReadHelper trait for SampleSizeBox
impl ReadHelper for SampleSizeBox {
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size()
    }

    fn total_size(&self) -> usize {
        let header_size = self.header.total_size(); // Size of BoxHeader
        let sample_count_size = SAMPLE_SIZE_BOX_SAMPLE_COUNT_SIZE; // Size of sample_count (4 bytes)
        let sample_size = self.sample_sizes.len() * SAMPLE_SIZE_BOX_SAMPLE_SIZE; // Variable size based on sample sizes

        // Total size is the sum of fixed sizes + variable size
        header_size + sample_count_size + sample_size
    }
}
