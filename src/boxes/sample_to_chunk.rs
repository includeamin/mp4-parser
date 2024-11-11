use super::header::BoxHeader;

const SAMPLE_TO_CHUNK_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const SAMPLE_TO_CHUNK_BOX_ENTRIES: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug, Clone)]
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
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let entry_count =
            u32::from_be_bytes(buffer[SAMPLE_TO_CHUNK_BOX_ENTRY_COUNT].try_into().unwrap());

        // For entries, it's variable-length, so we parse them.
        let mut entries = Vec::new();
        for chunk in buffer[SAMPLE_TO_CHUNK_BOX_ENTRIES.start..header.size()].chunks(12) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_to_chunk_box_basic() {
        // Prepare a valid buffer with 1 entry
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleToChunkBox (size = 32 bytes, type = "stsc")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b'c', b'c', // type = "stsc"
            // entry_count (4 bytes)
            0x00, 0x00, 0x00, 0x01, // entry_count = 1
            // entries (12 bytes)
            0x00, 0x00, 0x00, 0x01, // first_chunk = 1
            0x00, 0x00, 0x00, 0x01, // samples_per_chunk = 1
            0x00, 0x00, 0x00, 0x01, // sample_description_index = 1
        ];

        // Construct the SampleToChunkBox from the buffer
        let sample_to_chunk_box = SampleToChunkBox::from_buffer(&mock_buffer);

        // Test entry_count extraction
        assert_eq!(sample_to_chunk_box.get_entry_count(), 1);

        // Test entries extraction
        let expected_entries = [(1, 1, 1)];
        assert_eq!(sample_to_chunk_box.get_entries(), &expected_entries[..]);
    }

    #[test]
    fn test_sample_to_chunk_box_multiple_entries() {
        // Prepare a valid buffer with multiple entries (3 entries)
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleToChunkBox (size = 56 bytes, type = "stsc")
            0x00, 0x00, 0x00, 0x38, // size = 56
            b's', b't', b'c', b'c', // type = "stsc"
            // entry_count (4 bytes)
            0x00, 0x00, 0x00, 0x03, // entry_count = 3
            // entries (12 bytes each, 3 entries)
            0x00, 0x00, 0x00, 0x01, // first_chunk = 1
            0x00, 0x00, 0x00, 0x01, // samples_per_chunk = 1
            0x00, 0x00, 0x00, 0x01, // sample_description_index = 1
            0x00, 0x00, 0x00, 0x02, // first_chunk = 2
            0x00, 0x00, 0x00, 0x02, // samples_per_chunk = 2
            0x00, 0x00, 0x00, 0x02, // sample_description_index = 2
            0x00, 0x00, 0x00, 0x03, // first_chunk = 3
            0x00, 0x00, 0x00, 0x03, // samples_per_chunk = 3
            0x00, 0x00, 0x00, 0x03, // sample_description_index = 3
        ];

        // Construct the SampleToChunkBox from the buffer
        let sample_to_chunk_box = SampleToChunkBox::from_buffer(&mock_buffer);

        // Test entry_count extraction
        assert_eq!(sample_to_chunk_box.get_entry_count(), 3);

        // Test entries extraction
        let expected_entries = [(1, 1, 1), (2, 2, 2), (3, 3, 3)];
        assert_eq!(sample_to_chunk_box.get_entries(), &expected_entries[..]);
    }

    #[test]
    fn test_sample_to_chunk_box_no_entries() {
        // Prepare a valid buffer with no entries (entry_count = 0)
        let mock_buffer: Vec<u8> = vec![
            // BoxHeader for SampleToChunkBox (size = 32 bytes, type = "stsc")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b'c', b'c', // type = "stsc"
            // entry_count (4 bytes)
            0x00, 0x00, 0x00, 0x00, // entry_count = 0

                  // No entries (empty)
        ];

        // Construct the SampleToChunkBox from the buffer
        let sample_to_chunk_box = SampleToChunkBox::from_buffer(&mock_buffer);

        // Test entry_count extraction
        assert_eq!(sample_to_chunk_box.get_entry_count(), 0);

        // Test entries extraction (should be empty)
        let expected_entries: Vec<(u32, u32, u32)> = vec![];
        assert_eq!(sample_to_chunk_box.get_entries(), &expected_entries[..]);
    }

    #[test]
    fn test_sample_to_chunk_box_malformed_buffer() {
        // Prepare a malformed buffer (entry size is incorrect)
        let malformed_buffer: Vec<u8> = vec![
            // BoxHeader for SampleToChunkBox (size = 32 bytes, type = "stsc")
            0x00, 0x00, 0x00, 0x20, // size = 32
            b's', b't', b'c', b'c', // type = "stsc"
            // entry_count (4 bytes)
            0x00, 0x00, 0x00, 0x01, // entry_count = 1
            // malformed entry (not 12 bytes)
            0x00, 0x00, 0x00, 0x01, // first_chunk = 1
            0x00, 0x00, 0x00, 0x01, // samples_per_chunk = 1
        ];

        // Expect panic due to malformed buffer (entry size is not 12 bytes)
        let result = std::panic::catch_unwind(|| {
            SampleToChunkBox::from_buffer(&malformed_buffer);
        });

        assert!(result.is_err());
    }
}
