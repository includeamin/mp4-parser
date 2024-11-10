use crate::utils::{get_range, ReadHelper};

// Constants for range definitions
const HEADER_SIZE: std::ops::Range<usize> = 0..4;
const HEADER_NAME: std::ops::Range<usize> = 4..8;
const EXTENDED_SIZE: std::ops::Range<usize> = 8..16; // Range for extended size (if present)

#[derive(Debug, Clone)]
pub struct BoxHeader {
    size: u32,                  // 4 bytes at offset 0
    box_type: [u8; 4],          // 4 bytes at offset 4
    extended_size: Option<u64>, // 8 bytes at offset 8 if size == 0xFFFFFFFF (extended size)
}

impl BoxHeader {
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

impl ReadHelper for BoxHeader {
    /// Returns the end range (the position of the last byte) based on the `seek` position.
    ///
    /// # Parameters:
    /// - `seek`: The starting byte position in the buffer where the `BoxHeader` begins.
    ///
    /// # Returns:
    /// The end byte position (inclusive).
    fn get_end_range(&self, seek: usize) -> usize {
        // Calculate the size of the BoxHeader based on whether extended size is used
        let end_of_header = seek + self.total_size();
        end_of_header - 1 // Return the last byte position, inclusive
    }

    /// Returns the total size of the `BoxHeader` in bytes.
    ///
    /// # Returns:
    /// The total size of the `BoxHeader`.
    fn total_size(&self) -> usize {
        // The size is 4 bytes for the standard size field,
        // 4 bytes for the box type field,
        // and 8 bytes for the extended size if the size field is 0xFFFFFFFF.
        if self.extended_size.is_some() {
            16 // Extended size is present, so the total size is 16 bytes
        } else {
            8 // Only standard size and type fields are present, so the total size is 8 bytes
        }
    }
}
