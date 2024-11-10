use crate::utils::{get_range, get_start_seek, ReadHelper};

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
    pub fn from_buffer(header: &[u8]) -> Self {
        // Read the 4-byte size field at the seek offset
        let size = u32::from_be_bytes(header[HEADER_SIZE].try_into().unwrap());

        // Read the 4-byte box type
        let box_type: [u8; 4] = header[HEADER_NAME].try_into().unwrap();

        // Check if the size is 0xFFFFFFFF, which indicates the presence of extended_size
        let extended_size = if size == 0xFFFFFFFFu32 {
            // Read the 8-byte extended size if present
            Some(u64::from_be_bytes(
                header[EXTENDED_SIZE].try_into().unwrap(),
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
    pub fn size(&self) -> u32 {
        self.size
    }

    // Getter for `box_type`
    pub fn box_type(&self) -> String {
        String::from_utf8(self.box_type.to_vec()).unwrap()
    }

    // Getter for `extended_size`
    pub fn extended_size(&self) -> Option<u64> {
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
        seek + self.total_size()
    }

    /// Returns the total size of the `BoxHeader` in bytes.
    ///
    /// # Returns:
    /// The total size of the `BoxHeader`.
    fn total_size(&self) -> usize {
        if self.extended_size.is_some() {
            16 // Extended size is present, so the total size is 16 bytes
        } else {
            8 // Only standard size and type fields are present, so the total size is 8 bytes
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{get_range, ReadHelper};

    #[test]
    fn test_box_header_from_buffer_standard_size() {
        // Mock data buffer for a BoxHeader with a standard size (no extended size)
        let buffer: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x73, 0x74, 0x63, 0x6f, // box_type: "stco"
        ];

        let box_header = BoxHeader::from_buffer(&buffer);

        // Validate fields
        assert_eq!(box_header.size(), 8);
        assert_eq!(box_header.box_type(), "stco");
        assert_eq!(box_header.extended_size(), None);
        assert_eq!(box_header.total_size(), 8);
        assert_eq!(box_header.get_end_range(0), 8); // last byte position
    }

    #[test]
    fn test_box_header_from_buffer_extended_size() {
        // Mock data buffer for a BoxHeader with extended size
        let buffer: Vec<u8> = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // size: 0xFFFFFFFF (indicating extended size)
            0x73, 0x74, 0x63, 0x6f, // box_type: "stco"
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x20, // extended_size: 32
        ];

        let box_header = BoxHeader::from_buffer(&buffer);

        // Validate fields
        assert_eq!(box_header.size(), 0xFFFFFFFF);
        assert_eq!(box_header.box_type(), "stco");
        assert_eq!(box_header.extended_size(), Some(32));
        assert_eq!(box_header.total_size(), 16);
        assert_eq!(box_header.get_end_range(0), 16); // last byte position
    }

    #[test]
    fn test_box_header_total_size_standard() {
        let buffer: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x6d, 0x64, 0x61, 0x74, // box_type: "mdat"
        ];

        let box_header = BoxHeader::from_buffer(&buffer);
        assert_eq!(box_header.total_size(), 8);
    }

    #[test]
    fn test_box_header_total_size_extended() {
        let buffer: Vec<u8> = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // size: 0xFFFFFFFF
            0x66, 0x74, 0x79, 0x70, // box_type: "ftyp"
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // extended_size: 64
        ];

        let box_header = BoxHeader::from_buffer(&buffer);
        assert_eq!(box_header.total_size(), 16);
    }
}
