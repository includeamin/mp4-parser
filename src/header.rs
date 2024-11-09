use crate::utils::get_range;

const HEADER_SIZE: std::ops::Range<usize> = 0..4;
const HEADER_NAME: std::ops::Range<usize> = 4..8;

#[derive(Debug)]
pub struct BoxHeader {
    pub size: u32,                  // 4 bytes at offset 0; if 1, indicates an extended size
    pub box_type: [u8; 4], // 4 bytes at offset 4; identifies the box type, e.g., "ftyp", "moov"
    pub extended_size: Option<u64>, // 8 bytes at offset 8 if `size` is 1, otherwise None
}

impl BoxHeader {
    /// Constructs a `BoxHeader` with standard 32-bit size.
    fn new(size: u32, box_type: [u8; 4]) -> Self {
        Self {
            size,
            box_type,
            extended_size: if size == 1 { Some(0) } else { None },
        }
    }

    /// Constructs a `BoxHeader` with a 64-bit extended size.
    fn new_with_extended_size(extended_size: u64, box_type: [u8; 4]) -> Self {
        Self {
            size: 1, // Indicates that an extended size is used
            box_type,
            extended_size: Some(extended_size),
        }
    }

    /// Calculates the total size of the header in bytes, considering extended size if present.
    fn total_size(&self) -> u64 {
        if let Some(ext_size) = self.extended_size {
            ext_size // 64-bit extended size if specified
        } else {
            self.size as u64 // Default 32-bit size
        }
    }

    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        Self::new(
            u32::from_be_bytes(buffer[get_range(seek, HEADER_SIZE)].try_into().unwrap()),
            buffer[get_range(seek, HEADER_NAME)].try_into().unwrap(),
        )
    }

    pub fn get_box_type(&self) -> String {
        String::from_utf8(self.box_type.to_vec()).unwrap()
    }
}
