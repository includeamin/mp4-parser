use crate::utils::get_range;

// Constants for range definitions
const HEADER_SIZE: std::ops::Range<usize> = 0..4;
const HEADER_NAME: std::ops::Range<usize> = 4..8;
const EXTENDED_SIZE: std::ops::Range<usize> = 8..16; // Range for extended size (if present)

#[derive(Debug)]
pub struct BoxHeader {
    size: u32,                  // 4 bytes at offset 0
    box_type: [u8; 4],          // 4 bytes at offset 4
    extended_size: Option<u64>, // 8 bytes at offset 8 if size == 0xFFFFFFFF (extended size)
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
        // Read the standard 4-byte size field
        let size = u32::from_be_bytes(buffer[get_range(seek, HEADER_SIZE)].try_into().unwrap());

        // Read the 4-byte box type
        let box_type: [u8; 4] = buffer[get_range(seek, HEADER_NAME)].try_into().unwrap();

        // Check if the size is 0xFFFFFFFF, which indicates the presence of extended_size
        let extended_size = if size == 0xFFFFFFFF {
            // Read the 8-byte extended size if present
            Some(u64::from_be_bytes(
                buffer[get_range(seek, EXTENDED_SIZE)].try_into().unwrap(),
            ))
        } else {
            None
        };

        BoxHeader {
            size,
            box_type,
            extended_size,
        }
    }

    // Getter for `size`
    pub fn get_size(&self) -> u32 {
        self.size
    }

    // Getter for `box_type`
    pub fn get_box_type(&self) -> String {
        String::from_utf8(self.box_type.to_vec()).unwrap()
    }

    // Getter for `extended_size`
    pub fn get_extended_size(&self) -> Option<u64> {
        self.extended_size
    }
}
