use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from, ReadHelper};

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
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let entry_count = u32::from_be_bytes(
            buffer[get_range(seek, CHUNK_OFFSET_BOX_ENTRY_COUNT)]
                .try_into()
                .unwrap(),
        );

        // For chunk_offsets, it's variable-length, so we parse them.
        let chunk_offsets = buffer[get_range_from(seek, CHUNK_OFFSET_BOX_CHUNK_OFFSETS)]
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

/// Implementation of `ReadHelper` trait for `ChunkOffsetBox` to calculate the end index of the box.
impl ReadHelper for ChunkOffsetBox {
    fn get_end_range(&self, seek: usize) -> usize {
        // Use the `total_size` method to calculate the end index of the box
        seek + self.total_size()
    }

    /// Calculates and returns the total size of the `ChunkOffsetBox`.
    ///
    /// The total size is the size of the header, entry count (4 bytes), and the size of the chunk offsets array.
    /// The chunk offsets are variable-length, so we calculate their size based on the number of entries.
    ///
    /// # Returns
    /// The total size of the `ChunkOffsetBox` in bytes.
    fn total_size(&self) -> usize {
        // Size of the header (BoxHeader)
        let header_size = self.header.total_size();

        // Size of the entry count (4 bytes)
        let entry_count_size = SIZE_ENTRY_COUNT;

        // Size of the chunk_offsets array (each chunk offset is 4 bytes)
        let chunk_offsets_size = self.chunk_offsets.len() * SIZE_CHUNK_OFFSET_ENTRY;

        // Total size is the sum of all these components
        header_size + entry_count_size + chunk_offsets_size
    }
}
