use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from};

const SAMPLE_TO_CHUNK_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_TO_CHUNK_BOX_ENTRIES: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug)]
pub struct SampleToChunkBox {
    header: BoxHeader,             // Size and type at offset 0–7
    entry_count: u32,              // 4 bytes at offset 8–11
    entries: Vec<(u32, u32, u32)>, // Variable length: first_chunk, samples_per_chunk, sample_description_index
}

impl SampleToChunkBox {
    /// Constructs a `SampleToChunkBox` from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `seek` - The starting offset for reading the box.
    /// * `buffer` - The byte slice containing the MP4 data.
    ///
    /// # Returns
    ///
    /// A `SampleToChunkBox` constructed from the given buffer.
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let entry_count = u32::from_be_bytes(
            buffer[get_range(seek, SAMPLE_TO_CHUNK_BOX_ENTRY_COUNT)]
                .try_into()
                .unwrap(),
        );

        // For entries, it's variable-length, so we parse them.
        let mut entries = Vec::new();
        let entries_range = get_range_from(seek, SAMPLE_TO_CHUNK_BOX_ENTRIES);
        for chunk in buffer[entries_range].chunks(12) {
            let first_chunk = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let samples_per_chunk = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
            let sample_description_index = u32::from_be_bytes(chunk[8..12].try_into().unwrap());
            entries.push((first_chunk, samples_per_chunk, sample_description_index));
        }

        SampleToChunkBox {
            header,
            entry_count,
            entries,
        }
    }

    /// Getter for the `header` field.
    ///
    /// # Returns
    ///
    /// A reference to the `BoxHeader`.
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }

    /// Getter for the `entry_count` field.
    ///
    /// # Returns
    ///
    /// The number of entries in the `SampleToChunkBox`.
    pub fn get_entry_count(&self) -> u32 {
        self.entry_count
    }

    /// Getter for the `entries` field.
    ///
    /// # Returns
    ///
    /// A reference to the list of entries, where each entry is a tuple
    /// of (first_chunk, samples_per_chunk, sample_description_index).
    pub fn get_entries(&self) -> &[(u32, u32, u32)] {
        &self.entries
    }
}
