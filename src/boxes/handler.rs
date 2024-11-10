use super::header::BoxHeader;

// consts for HandlerBox
const HANDLER_BOX_VERSION: std::ops::Range<usize> = 8..9;
const HANDLER_BOX_FLAGS: std::ops::Range<usize> = 9..12;
const HANDLER_BOX_HANDLER_TYPE: std::ops::Range<usize> = 12..16;
const HANDLER_BOX_RESERVED: std::ops::Range<usize> = 16..28;
const HANDLER_BOX_NAME_START: std::ops::RangeFrom<usize> = 28..; // Null-terminated, variable length

/// Represents the `HandlerBox` in an MP4 container file.
///
/// This box contains information about the handler type (e.g., video, audio) and related metadata.
#[derive(Debug, Clone)]
pub struct HandlerBox {
    header: BoxHeader,     // Size and type at offset 0–7
    version: u8,           // 1 byte at offset 8
    flags: [u8; 3],        // 3 bytes at offset 9–11
    handler_type: [u8; 4], // 4 bytes at offset 12–15 (e.g., 'vide' for video, 'soun' for audio)
    reserved: [u8; 12],    // 12 bytes reserved at offset 16–27
    name: Vec<u8>,         // Null-terminated string starting at offset 28
}

impl HandlerBox {
    /// Creates a new `HandlerBox` instance by parsing the raw byte buffer starting from the given `seek` index.
    ///
    /// # Arguments
    /// * `seek` - The index to start reading from in the buffer.
    /// * `buffer` - The raw byte buffer containing the `HandlerBox` data.
    ///
    /// # Returns
    /// A `HandlerBox` instance populated with data extracted from the buffer.
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let version = buffer[HANDLER_BOX_VERSION][0];
        let flags = [
            buffer[HANDLER_BOX_FLAGS][0],
            buffer[HANDLER_BOX_FLAGS][1],
            buffer[HANDLER_BOX_FLAGS][2],
        ];
        let handler_type = buffer[HANDLER_BOX_HANDLER_TYPE].try_into().unwrap();
        let reserved = buffer[HANDLER_BOX_RESERVED].try_into().unwrap();

        // For `name`, find the null-terminated string starting from `HANDLER_BOX_NAME_START`
        let name_start = HANDLER_BOX_NAME_START.start;
        let name_end = buffer[name_start..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| name_start + pos)
            .unwrap_or(buffer.len());
        let name = buffer[name_start..name_end].to_vec();

        HandlerBox {
            header,
            version,
            flags,
            handler_type,
            reserved,
            name,
        }
    }

    /// Returns the header information of the `HandlerBox`.
    ///
    /// # Returns
    /// The `BoxHeader` which contains the size and type of the box.
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    /// Returns the version of the `HandlerBox`.
    ///
    /// # Returns
    /// The version byte at offset 8.
    pub fn version(&self) -> u8 {
        self.version
    }

    /// Returns the flags of the `HandlerBox`.
    ///
    /// # Returns
    /// A 3-byte array containing the flags at offset 9–11.
    pub fn flags(&self) -> &[u8; 3] {
        &self.flags
    }

    /// Returns the handler type of the `HandlerBox` (e.g., 'vide' for video, 'soun' for audio).
    ///
    /// # Returns
    /// A 4-byte array representing the handler type at offset 12–15.
    pub fn handler_type(&self) -> &[u8; 4] {
        &self.handler_type
    }

    /// Returns the reserved bytes in the `HandlerBox`.
    ///
    /// # Returns
    /// A 12-byte array of reserved data at offset 16–27.
    pub fn reserved(&self) -> &[u8; 12] {
        &self.reserved
    }

    /// Returns the name associated with the `HandlerBox` as a UTF-8 string.
    ///
    /// # Returns
    /// The name as a `String`, parsed from the null-terminated byte sequence starting at offset 28.
    pub fn name(&self) -> String {
        String::from_utf8_lossy(&self.name).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_buffer() -> Vec<u8> {
        let mut buffer = vec![];

        // Mock BoxHeader (assuming 8 bytes total: size (4 bytes), type (4 bytes))
        buffer.extend_from_slice(&[0, 0, 0, 41]); // size = 41 bytes (adjusted)
        buffer.extend_from_slice(b"hdlr"); // type = "hdlr"

        // Mock version and flags (4 bytes total)
        buffer.push(1); // version
        buffer.extend_from_slice(&[0, 0, 1]); // flags

        // Mock handler_type (4 bytes)
        buffer.extend_from_slice(b"vide"); // handler_type = "vide"

        // Mock reserved (12 bytes)
        buffer.extend_from_slice(&[0; 12]); // reserved

        // Mock name (null-terminated string)
        buffer.extend_from_slice(b"Test Handler");
        buffer.push(0); // null-terminator

        buffer
    }

    #[test]
    fn test_handler_box_from_buffer() {
        let buffer = create_test_buffer();
        let handler_box = HandlerBox::from_buffer(&buffer);

        // Verify header
        assert_eq!(handler_box.header().box_type(), "hdlr");
        assert_eq!(handler_box.header().size(), 41);

        // Verify version and flags
        assert_eq!(handler_box.version(), 1);
        assert_eq!(handler_box.flags(), &[0, 0, 1]);

        // Verify handler type
        assert_eq!(handler_box.handler_type(), b"vide");

        // Verify reserved
        assert_eq!(handler_box.reserved(), &[0; 12]);

        // Verify name
        assert_eq!(handler_box.name(), "Test Handler".to_string());
    }

    #[test]
    fn test_handler_box_total_size() {
        let buffer = create_test_buffer();
        let handler_box = HandlerBox::from_buffer(&buffer);

        // Expected size = header (8) + version (1) + flags (3) + handler_type (4) + reserved (12) + name ("Test Handler" + null-terminator = 13)
        let expected_size = 8 + 1 + 3 + 4 + 12 + 13;
        assert_eq!(handler_box.header().size() as usize, expected_size);
        assert_eq!(handler_box.header().size() as usize, expected_size);
    }

    #[test]
    fn test_name_null_terminated_handling() {
        // Test with different name lengths and ensure null termination is respected
        let mut buffer = create_test_buffer();

        // Add extra data beyond the null terminator
        buffer.extend_from_slice(b"Extra data after null");

        let handler_box = HandlerBox::from_buffer(&buffer);

        // Verify that name extraction stops at the null terminator
        assert_eq!(handler_box.name(), "Test Handler".to_string());
    }
}
