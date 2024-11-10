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
    pub fn size(&self) -> usize {
        self.size as usize
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

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(box_header.size(), 8);
    }

    #[test]
    fn test_box_header_from_buffer_extended_size() {
        // Mock data buffer for a BoxHeader with extended size
        let buffer: Vec<u8> = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // Size field indicating extended size
            0x73, 0x74, 0x63, 0x6F, // Box type "stco" (4 bytes)
            // Extended size: 48 bytes
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x30, // Actual box size (48 bytes)
            // Entry count: 2 (4 bytes)
            0x00, 0x00, 0x00, 0x02, // entry_count = 2
            // Chunk offsets: 2 entries (each 4 bytes)
            0x00, 0x00, 0x00, 0x10, // Chunk offset 1 (16 bytes)
            0x00, 0x00, 0x00, 0x20, // Chunk offset 2 (32 bytes)
        ];

        let box_header = BoxHeader::from_buffer(&buffer);

        // Validate fields
        assert_eq!(box_header.size(), 0xFFFFFFFF);
        assert_eq!(box_header.box_type(), "stco");
        assert_eq!(box_header.extended_size(), Some(48));
        assert_eq!(box_header.size(), 4294967295);
    }

    #[test]
    fn test_box_header_total_size_standard() {
        let buffer: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x08, // size: 8
            0x6d, 0x64, 0x61, 0x74, // box_type: "mdat"
        ];

        let box_header = BoxHeader::from_buffer(&buffer);
        assert_eq!(box_header.size(), 8);
    }

    #[test]
    fn test_box_header_total_size_extended() {
        let buffer: Vec<u8> = vec![
            0xFF, 0xFF, 0xFF, 0xFF, // size: 0xFFFFFFFF
            0x66, 0x74, 0x79, 0x70, // box_type: "ftyp"
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, // extended_size: 64
        ];

        let box_header = BoxHeader::from_buffer(&buffer);
        assert_eq!(box_header.size(), 4294967295);
    }
}
