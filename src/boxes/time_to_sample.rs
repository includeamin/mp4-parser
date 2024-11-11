use super::header::BoxHeader;

const TIME_TO_SAMPLE_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const TIME_TO_SAMPLE_BOX_ENTRIES: std::ops::RangeFrom<usize> = 12..;

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

        // Parse entry count (4 bytes at position TIME_TO_SAMPLE_BOX_ENTRY_COUNT)
        let entry_count =
            u32::from_be_bytes(buffer[TIME_TO_SAMPLE_BOX_ENTRY_COUNT].try_into().unwrap());

        // Now parse the entries, assuming entries start after entry_count
        let mut entries = Vec::new();

        // Slice the entry data and parse each entry (sample_count + duration)
        let entry_data = &buffer[TIME_TO_SAMPLE_BOX_ENTRIES.start..header.size()];
        for chunk in entry_data.chunks(8) {
            if chunk.len() == 8 {
                let sample_count = u32::from_be_bytes(chunk[0..4].try_into().unwrap());
                let duration = u32::from_be_bytes(chunk[4..8].try_into().unwrap());
                entries.push((sample_count, duration));
            } else {
                panic!("Invalid chunk size: expected 8 bytes, got {}", chunk.len());
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_valid_mock_buffer() -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::new();

        // BoxHeader for "stts" (size = 28 bytes, type = "stts")
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x1C]); // Size (28 bytes, header + data)
        buffer.extend_from_slice(b"stts"); // Type = "stts"

        // Entry count (4 bytes, 2 entries)
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x02]); // entry_count = 2

        // TimeToSample entries (8 bytes each)
        // First entry: sample_count = 10, duration = 20
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x0A]); // sample_count = 10
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x14]); // duration = 20

        // Second entry: sample_count = 30, duration = 40
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x1E]); // sample_count = 30
        buffer.extend_from_slice(&[0x00, 0x00, 0x00, 0x28]); // duration = 40

        buffer
    }

    #[test]
    fn test_from_buffer() {
        // Create the valid mock buffer
        let buffer = create_valid_mock_buffer();

        // Parse the TimeToSampleBox from the buffer
        let time_to_sample_box = TimeToSampleBox::from_buffer(&buffer);

        // Verify the header
        let header = time_to_sample_box.get_header();
        assert_eq!(header.size(), 28); // Size of the box (BoxHeader + data)
        assert_eq!(header.box_type(), "stts"); // Type should be "stts"

        // Verify the entry count
        assert_eq!(time_to_sample_box.get_entry_count(), 2); // Two entries

        // Verify the entries
        let entries = time_to_sample_box.get_entries();
        assert_eq!(entries.len(), 2); // Two entries
        assert_eq!(entries[0], (10, 20)); // First entry: (sample_count = 10, duration = 20)
        assert_eq!(entries[1], (30, 40)); // Second entry: (sample_count = 30, duration = 40)
    }

    #[test]
    fn test_total_size() {
        // Create the valid mock buffer
        let buffer = create_valid_mock_buffer();

        // Parse the TimeToSampleBox from the buffer
        let time_to_sample_box = TimeToSampleBox::from_buffer(&buffer);

        // Calculate total size (header size + entry count size + entry size)
        let expected_size = 28; // header (8) + entry_count (4) + 2 entries (8*2)
        assert_eq!(time_to_sample_box.get_header().size(), expected_size);
    }
}
