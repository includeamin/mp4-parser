use crate::utils::get_range;

const HEADER_SIZE: std::ops::Range<usize> = 0..4;
const HEADER_NAME: std::ops::Range<usize> = 4..8;

#[derive(Debug)]
pub struct BoxHeader {
    pub size: u32,
    pub name: String,
}

impl BoxHeader {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        // TODO: check for size
        Self {
            size: u32::from_be_bytes(buffer[get_range(seek, HEADER_SIZE)].try_into().unwrap()),
            name: String::from_utf8(buffer[get_range(seek, HEADER_NAME)].into())
                .unwrap(),
        }
    }
}
