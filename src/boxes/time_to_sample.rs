use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from, ReadHelper};

const TIME_TO_SAMPLE_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const TIME_TO_SAMPLE_BOX_ENTRIES: std::ops::RangeFrom<usize> = 12..;

// Constants for fixed sizes
const TIME_TO_SAMPLE_BOX_ENTRY_COUNT_SIZE: usize = 4; // 4 bytes for entry_count
const TIME_TO_SAMPLE_BOX_ENTRY_SIZE: usize = 8; // 8 bytes for each entry (sample_count + duration)

#[derive(Debug, Clone)]
pub struct TimeToSampleBox {
    header: BoxHeader,        // Size and type at offset 0–7
    entry_count: u32,         // 4 bytes at offset 8–11
    entries: Vec<(u32, u32)>, // Variable length: each entry has sample count and duration
}

impl TimeToSampleBox {
    /// Constructs a `TimeToSampleBox` from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `seek` - The starting offset for reading the box.
    /// * `buffer` - The byte slice containing the MP4 data.
    ///
    /// # Returns
    ///
    /// A `TimeToSampleBox` constructed from the given buffer.
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let entry_count =
            u32::from_be_bytes(buffer[TIME_TO_SAMPLE_BOX_ENTRY_COUNT].try_into().unwrap());

        // For entries, it's variable-length, so we parse them.
        let mut entries = Vec::new();
        for chunk in buffer[TIME_TO_SAMPLE_BOX_ENTRIES].chunks(8) {
            let sample_count = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
            let duration = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
            entries.push((sample_count, duration));
        }

        TimeToSampleBox {
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
    /// The number of entries in the `TimeToSampleBox`.
    pub fn get_entry_count(&self) -> u32 {
        self.entry_count
    }

    /// Getter for the `entries` field.
    ///
    /// # Returns
    ///
    /// A reference to the list of entries, where each entry is a tuple
    /// of (sample_count, duration).
    pub fn get_entries(&self) -> &[(u32, u32)] {
        &self.entries
    }
}

// Implementing ReadHelper trait for TimeToSampleBox
impl ReadHelper for TimeToSampleBox {
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size()
    }

    fn total_size(&self) -> usize {
        let header_size = self.header.total_size(); // Size of the BoxHeader
        let entry_count_size = TIME_TO_SAMPLE_BOX_ENTRY_COUNT_SIZE; // Size of entry_count (4 bytes)
        let entry_size = self.entries.len() * TIME_TO_SAMPLE_BOX_ENTRY_SIZE; // Variable size based on entries

        // Total size is the sum of fixed sizes + variable size
        header_size + entry_count_size + entry_size
    }
}
