use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from, ReadHelper};

// consts for HandlerBox
const HANDLER_BOX_VERSION: std::ops::Range<usize> = 8..9;
const HANDLER_BOX_FLAGS: std::ops::Range<usize> = 9..12;
const HANDLER_BOX_HANDLER_TYPE: std::ops::Range<usize> = 12..16;
const HANDLER_BOX_RESERVED: std::ops::Range<usize> = 16..28;
const HANDLER_BOX_NAME_START: std::ops::RangeFrom<usize> = 28..; // Null-terminated, variable length

const HANDLER_BOX_VERSION_SIZE: usize = 1; // 1 byte at offset 8
const HANDLER_BOX_FLAGS_SIZE: usize = 3; // 3 bytes at offset 9–11
const HANDLER_BOX_HANDLER_TYPE_SIZE: usize = 4; // 4 bytes at offset 12–15
const HANDLER_BOX_RESERVED_SIZE: usize = 12; // 12 bytes at offset 16–27

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

impl ReadHelper for HandlerBox {
    /// Returns the end range (the index) of the `HandlerBox`, calculated based on the size of the box.
    ///
    /// # Arguments
    /// * `seek` - The starting index in the buffer where the `HandlerBox` begins.
    ///
    /// # Returns
    /// The end index in the buffer for the `HandlerBox`, calculated as the starting index plus the total size of the box.
    fn get_end_range(&self, seek: usize) -> usize {
        // The end range is simply the starting index plus the total size of the box.
        seek + self.total_size()
    }

    /// Calculates and returns the total size of the `HandlerBox`.
    ///
    /// The total size is calculated as the sum of the header size, version size, flags size, handler type size,
    /// reserved size, and the size of the name (which is variable-length).
    ///
    /// # Returns
    /// The total size of the `HandlerBox` in bytes.
    fn total_size(&self) -> usize {
        // Size of the header (BoxHeader)
        let header_size = self.header.total_size();

        // Size of the name (variable length, calculated from the null-terminated string length)
        let name_size = self.name.len();

        // Total size is the sum of all these components
        header_size
            + HANDLER_BOX_VERSION_SIZE
            + HANDLER_BOX_FLAGS_SIZE
            + HANDLER_BOX_HANDLER_TYPE_SIZE
            + HANDLER_BOX_RESERVED_SIZE
            + name_size
    }
}
