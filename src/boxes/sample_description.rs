use crate::utils::{get_range, get_range_from};

use super::header::BoxHeader;
const SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug)]
pub struct SampleDescriptionBox {
    header: BoxHeader,           // Size and type at offset 0–7
    sample_count: u32,           // 4 bytes at offset 8–11
    sample_description: Vec<u8>, // Variable length after offset 12
}

impl SampleDescriptionBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let sample_count = u32::from_be_bytes(
            buffer[get_range(seek, SAMPLE_DESCRIPTION_BOX_SAMPLE_COUNT)]
                .try_into()
                .unwrap(),
        );

        // For sample_description, it's a variable-length field.
        let sample_description =
            buffer[get_range_from(seek, SAMPLE_DESCRIPTION_BOX_SAMPLE_DESCRIPTION)].to_vec();

        SampleDescriptionBox {
            header,
            sample_count,
            sample_description,
        }
    }
}
