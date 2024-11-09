use crate::utils::{get_range, get_range_from};

use super::header::BoxHeader;
const SAMPLE_SIZE_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_SIZE_BOX_SAMPLE_SIZES: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug)]
pub struct SampleSizeBox {
    header: BoxHeader,      // Size and type at offset 0–7
    sample_count: u32,      // 4 bytes at offset 8–11
    sample_sizes: Vec<u32>, // Variable length after offset 12
}

impl SampleSizeBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let sample_count = u32::from_be_bytes(
            buffer[get_range(seek, SAMPLE_SIZE_BOX_SAMPLE_COUNT)]
                .try_into()
                .unwrap(),
        );

        // For sample_sizes, it's variable-length, so we parse them.
        let sample_sizes = buffer[get_range_from(seek, SAMPLE_SIZE_BOX_SAMPLE_SIZES)]
            .chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();

        SampleSizeBox {
            header,
            sample_count,
            sample_sizes,
        }
    }
}
