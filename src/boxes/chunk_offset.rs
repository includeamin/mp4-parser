use super::header::BoxHeader;

// Constants for sizes
const SIZE_ENTRY_COUNT: usize = 4; // Size of the entry count (4 bytes)
const SIZE_CHUNK_OFFSET_ENTRY: usize = 4; // Size of each chunk offset (4 bytes)
const CHUNK_OFFSET_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const CHUNK_OFFSET_BOX_CHUNK_OFFSETS: std::ops::RangeFrom<usize> = 12..;

/// Represents the `ChunkOffsetBox` in an MP4 container file.
///
/// This box contains the number of entries and the chunk offsets, which are used to map
/// chunk data within the file.
#[derive(Debug, Clone)]
pub struct ChunkOffsetBox {
    header: BoxHeader,       // Size and type at offset 0–7
    entry_count: u32,        // 4 bytes at offset 8–11
    chunk_offsets: Vec<u32>, // Variable length after offset 12
}

impl ChunkOffsetBox {
    /// Creates a new `ChunkOffsetBox` instance by parsing the raw byte buffer starting from the given `seek` index.
    ///
    /// # Arguments
    /// * `seek` - The index to start reading from in the buffer.
    /// * `buffer` - The raw byte buffer containing the `ChunkOffsetBox` data.
    ///
    /// # Returns
    /// A `ChunkOffsetBox` instance populated with data extracted from the buffer.
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let entry_count =
            u32::from_be_bytes(buffer[CHUNK_OFFSET_BOX_ENTRY_COUNT].try_into().unwrap());

        // For chunk_offsets, it's variable-length, so we parse them.
        let chunk_offsets = buffer[CHUNK_OFFSET_BOX_CHUNK_OFFSETS]
            .chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();

        ChunkOffsetBox {
            header,
            entry_count,
            chunk_offsets,
        }
    }

    /// Returns the number of entries in the `ChunkOffsetBox`.
    ///
    /// # Returns
    /// The entry count (4 bytes at offset 8–11).
    pub fn entry_count(&self) -> u32 {
        self.entry_count
    }

    /// Returns the chunk offsets in the `ChunkOffsetBox`.
    ///
    /// # Returns
    /// A reference to a vector of chunk offsets (each 4 bytes) starting from offset 12.
    pub fn chunk_offsets(&self) -> &Vec<u32> {
        &self.chunk_offsets
    }

    /// Returns the header information of the `ChunkOffsetBox`.
    ///
    /// # Returns
    /// The `BoxHeader` which contains the size and type of the box.
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_offset_box_from_buffer() {
        // Define a buffer that includes a BoxHeader, entry count, and chunk offsets.
        let buffer: &[u8] = &[
            // BoxHeader: size = 28, type = "stco"
            0x00, 0x00, 0x00, 0x1C, // Size field (4 bytes)
            0x73, 0x74, 0x63, 0x6F, // Type field "stco" (4 bytes)
            // Entry count = 2
            0x00, 0x00, 0x00, 0x02, // Entry count (4 bytes)
            // Chunk offsets
            0x00, 0x00, 0x00, 0x10, // First chunk offset (4 bytes)
            0x00, 0x00, 0x00, 0x20, // Second chunk offset (4 bytes)
        ];

        // Parse the buffer
        let chunk_offset_box = ChunkOffsetBox::from_buffer(buffer);

        // Check header
        assert_eq!(chunk_offset_box.header().size(), 28);
        assert_eq!(chunk_offset_box.header().box_type(), "stco");

        // Check entry count
        assert_eq!(chunk_offset_box.entry_count(), 2);

        // Check chunk offsets
        let expected_offsets = vec![16, 32];
        assert_eq!(chunk_offset_box.chunk_offsets(), &expected_offsets);
    }

    #[test]
    fn test_chunk_offset_box_total_size() {
        // Define a buffer similar to the one above
        let buffer: &[u8] = &[
            0x00, 0x00, 0x00, 0x1C, // Size field (28 bytes total: header + data)
            0x73, 0x74, 0x63, 0x6F, // Type "stco" (4 bytes)
            0x00, 0x00, 0x00, 0x02, // Entry count = 2 (4 bytes)
            0x00, 0x00, 0x00, 0x10, // Chunk offset 1 = 16 (4 bytes)
            0x00, 0x00, 0x00, 0x20, // Chunk offset 2 = 32 (4 bytes)
        ];

        let chunk_offset_box = ChunkOffsetBox::from_buffer(buffer);

        assert_eq!(chunk_offset_box.header().size(), 28);
    }
}
