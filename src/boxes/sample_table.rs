use super::header::BoxHeader;
use super::{
    chunk_offset::ChunkOffsetBox, sample_description::SampleDescriptionBox,
    sample_size::SampleSizeBox, sample_to_chunk::SampleToChunkBox, time_to_sample::TimeToSampleBox,
};

/// Represents the `SampleTableBox` (stbl) in the MP4 file format.
#[derive(Debug, Clone)]
pub struct SampleTableBox {
    header: BoxHeader,          // Size and type at offset 0–7
    stsd: SampleDescriptionBox, // Sample Description Box (stsd)
    stts: TimeToSampleBox,      // Time-to-Sample Box (stts)
    stsc: SampleToChunkBox,     // Sample-to-Chunk Box (stsc)
    stsz: SampleSizeBox,        // Sample Size Box (stsz)
    stco: ChunkOffsetBox,       // Chunk Offset Box (stco)
}

impl SampleTableBox {
    /// Constructs a `SampleTableBox` from the provided buffer.
    ///
    /// # Arguments
    ///
    /// * `seek` - The starting offset for reading the box.
    /// * `buffer` - The byte slice containing the MP4 data.
    ///
    /// # Returns
    ///
    /// A `SampleTableBox` constructed from the given buffer.
    // pub fn from_buffer(buffer: &[u8]) -> Self {
    //     let header = BoxHeader::from_buffer(buffer);
    //     if buffer.len() < 8 {
    //         panic!("Buffer too small to contain stsd data");
    //     }
    //
    //     let stsd_ofset = 8;
    //     let stsd = SampleDescriptionBox::from_buffer(&buffer[stsd_ofset..]);
    //
    //     let stts_offset = stsd_ofset + stsd.get_header().size();
    //     if buffer.len() < stts_offset {
    //         panic!("Buffer too small to contain stts data");
    //     }
    //     let stts_data = &buffer[stts_offset..];
    //     let stts = TimeToSampleBox::from_buffer(stts_data);
    //
    //     let stsc_offset = stts_offset + stts.get_header().size();
    //     if buffer.len() < stsc_offset {
    //         panic!("Buffer too small to contain stsc data");
    //     }
    //     let stsc_data = &buffer[stsc_offset..];
    //     let stsc = SampleToChunkBox::from_buffer(stsc_data);
    //
    //     let stsz_offset = stsc_offset + stsc.get_header().size();
    //     if buffer.len() < stsz_offset {
    //         panic!("Buffer too small to contain stsz data");
    //     }
    //     let stsz_data = &buffer[stsz_offset..];
    //     let stsz = SampleSizeBox::from_buffer(stsz_data);
    //
    //     let stco_offset = stsz_offset + stsz.get_header().size();
    //     if buffer.len() < stco_offset {
    //         panic!("Buffer too small to contain stco data");
    //     }
    //     let stco_data = &buffer[stco_offset..];
    //     let stco = ChunkOffsetBox::from_buffer(stco_data);
    //
    //     SampleTableBox {
    //         header,
    //         stsd,
    //         stts,
    //         stsc,
    //         stsz,
    //         stco,
    //     }
    // }

    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        if buffer.len() < 8 {
            panic!("Buffer too small to contain stsd data");
        }

        // Determine version to correctly parse stbl content.
        let version = buffer[8]; // Version at offset 8
        let mut stsd_offset = 8; // Adjust offset for stsd based on version

        if version == 0 {
            // For version 0 (32-bit fields)
            stsd_offset += 20;
        } else if version == 1 {
            // For version 1 (64-bit fields)
            stsd_offset += 36;
        } else {
            panic!("Unsupported version");
        }

        // Parse SampleDescriptionBox (stsd)
        let stsd = SampleDescriptionBox::from_buffer(&buffer[stsd_offset..]);

        // Parse TimeToSampleBox (stts)
        let stts_offset = stsd_offset + stsd.get_header().size();
        let stts_data = &buffer[stts_offset..];
        let stts = TimeToSampleBox::from_buffer(stts_data);

        // Parse SampleToChunkBox (stsc)
        let stsc_offset = stts_offset + stts.get_header().size();
        let stsc_data = &buffer[stsc_offset..];
        let stsc = SampleToChunkBox::from_buffer(stsc_data);

        // Parse SampleSizeBox (stsz)
        let stsz_offset = stsc_offset + stsc.get_header().size();
        let stsz_data = &buffer[stsz_offset..];
        let stsz = SampleSizeBox::from_buffer(stsz_data);

        // Parse ChunkOffsetBox (stco)
        let stco_offset = stsz_offset + stsz.get_header().size();
        let stco_data = &buffer[stco_offset..];
        let stco = ChunkOffsetBox::from_buffer(stco_data);

        SampleTableBox {
            header,
            stsd,
            stts,
            stsc,
            stsz,
            stco,
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

    /// Getter for the `stsd` (Sample Description Box).
    ///
    /// # Returns
    ///
    /// A reference to the `SampleDescriptionBox`.
    pub fn get_sample_description(&self) -> &SampleDescriptionBox {
        &self.stsd
    }

    /// Getter for the `stts` (Time-to-Sample Box).
    ///
    /// # Returns
    ///
    /// A reference to the `TimeToSampleBox`.
    pub fn get_time_to_sample(&self) -> &TimeToSampleBox {
        &self.stts
    }

    /// Getter for the `stsc` (Sample-to-Chunk Box).
    ///
    /// # Returns
    ///
    /// A reference to the `SampleToChunkBox`.
    pub fn get_sample_to_chunk(&self) -> &SampleToChunkBox {
        &self.stsc
    }

    /// Getter for the `stsz` (Sample Size Box).
    ///
    /// # Returns
    ///
    /// A reference to the `SampleSizeBox`.
    pub fn get_sample_size(&self) -> &SampleSizeBox {
        &self.stsz
    }

    /// Getter for the `stco` (Chunk Offset Box).
    ///
    /// # Returns
    ///
    /// A reference to the `ChunkOffsetBox`.
    pub fn get_chunk_offset(&self) -> &ChunkOffsetBox {
        &self.stco
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOCK_MP4_BUFFER: &[u8] = &[
        // SampleTableBox (stbl) - Outer box
        0x00, 0x00, 0x00, 0x88, // Box size (163 bytes total) - u32
        0x73, 0x74, 0x62, 0x6C, // Box type ("stbl") - 4 bytes
        // SampleDescriptionBox (stsd) - Sub-box
        0x00, 0x00, 0x00, 0x10, // Size field (16 bytes) - u32
        0x73, 0x74, 0x73, 0x64, // Type field ("stsd") - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Version and flags - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Entry count (1) - u32
        // TimeToSampleBox (stts) - Sub-box
        0x00, 0x00, 0x00, 0x24, // Box size (36 bytes) - u32
        0x73, 0x74, 0x74, 0x73, // Box type ("stts") - 4 bytes
        0x00, 0x00, 0x00, 0x04, // Entry count (4 entries) - u32
        // Entry 1
        0x00, 0x00, 0x00, 0x0A, // Sample count (10) - u32
        0x00, 0x00, 0x00, 0x64, // Sample delta (100) - u32
        // Entry 2
        0x00, 0x00, 0x00, 0x05, // Sample count (5) - u32
        0x00, 0x00, 0x00, 0xC8, // Sample delta (200) - u32
        // Entry 3
        0x00, 0x00, 0x00, 0x08, // Sample count (8) - u32
        0x00, 0x00, 0x01, 0x2C, // Sample delta (300) - u32
        // SampleToChunkBox (stsc) - Sub-box
        0x00, 0x00, 0x00, 0x24, // Box size (36 bytes) - u32
        0x73, 0x74, 0x73, 0x63, // Type field ("stsc") - 4 bytes
        0x00, 0x00, 0x00, 0x03, // Entry count (3) - u32
        // Entry 1
        0x00, 0x00, 0x00, 0x01, // First chunk (1) - u32
        0x00, 0x00, 0x00, 0x64, // Samples per chunk (100) - u32
        // Entry 2
        0x00, 0x00, 0x00, 0x02, // Second chunk (2) - u32
        0x00, 0x00, 0x00, 0xC8, // Samples per chunk (200) - u32
        // Entry 3
        0x00, 0x00, 0x00, 0x03, // Third chunk (3) - u32
        0x00, 0x00, 0x01, 0x2C, // Samples per chunk (300) - u32
        // SampleSizeBox (stsz) - Sub-box
        0x00, 0x00, 0x00, 0x18, // Box size (24 bytes) - u32
        0x73, 0x74, 0x73, 0x7A, // Box type ("stsz") - 4 bytes
        0x00, 0x00, 0x00, 0x01, // Sample size entry count (1) - u32
        0x00, 0x00, 0x04, 0x00, // Sample size (1024) - u32
        0x00, 0x00, 0x00, 0x05, // Sample count (5) - u32
        // Sample sizes
        0x00, 0x00, 0x04, 0x00, // Sample size (1024) - u32
        // ChunkOffsetBox (stco) - Sub-box
        0x00, 0x00, 0x00, 0x10, // Size field (16 bytes) - u32
        0x73, 0x74, 0x63, 0x6F, // Type field ("stco") - 4 bytes
        0x00, 0x00, 0x00, 0x02, // Entry count (2) - u32
        // Chunk offsets
        0x00, 0x00, 0x00, 0x20, // Offset for chunk 1 (32 bytes) - u32
        0x00, 0x00, 0x00, 0x40, // Offset for chunk 2 (64 bytes) - u32
    ];

    #[test]
    fn test_sample_table_box_creation() {
        // Generate the mock MP4 buffer
        let buffer = MOCK_MP4_BUFFER;

        // Build the SampleTableBox from the buffer
        let stbl_box = SampleTableBox::from_buffer(buffer);

        // Verify the header was correctly parsed
        assert_eq!(stbl_box.get_header().size(), 136); // Assert that the header size is correct
        assert_eq!(stbl_box.get_header().box_type(), "stbl"); // Assert the box type is correct

        // Check the first sub-box (SampleDescriptionBox)
        let stsd_box = stbl_box.get_sample_description();
        assert_eq!(stsd_box.get_header().size(), 16); // Verify the size of the SampleDescriptionBox
        assert_eq!(stsd_box.get_sample_count(), 1); // Check entry count in the SampleDescriptionBox

        // Check the second sub-box (TimeToSampleBox)
        let stts_box = stbl_box.get_time_to_sample();
        assert_eq!(stts_box.get_header().size(), 36); // Verify the size of the TimeToSampleBox
        assert_eq!(stts_box.get_entry_count(), 4); // Check the entry count in TimeToSampleBox

        // Check the third sub-box (SampleToChunkBox)
        let stsc_box = stbl_box.get_sample_to_chunk();
        assert_eq!(stsc_box.get_header().size(), 36); // Verify the size of the SampleToChunkBox
        assert_eq!(stsc_box.get_entry_count(), 3); // Check the entry count in SampleToChunkBox

        // Check the fourth sub-box (SampleSizeBox)
        let stsz_box = stbl_box.get_sample_size();
        assert_eq!(stsz_box.get_header().size(), 24); // Verify the size of the SampleSizeBox
        assert_eq!(stsz_box.get_sample_sizes()[0], 1024); // Verify the sample size value

        // Check the fifth sub-box (ChunkOffsetBox)
        let stco_box = stbl_box.get_chunk_offset();
        assert_eq!(stco_box.header().size(), 16); // Verify the size of the ChunkOffsetBox
        assert_eq!(stco_box.entry_count(), 2); // Check the entry count in ChunkOffsetBox
    }
}
