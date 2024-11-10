use crate::utils::ReadHelper;

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
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let stsd = SampleDescriptionBox::from_buffer(buffer);
        let stts = TimeToSampleBox::from_buffer(buffer);
        let stsc = SampleToChunkBox::from_buffer(buffer);
        let stsz = SampleSizeBox::from_buffer(buffer);
        let stco = ChunkOffsetBox::from_buffer(buffer);

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

// Implementing ReadHelper trait for SampleTableBox
impl ReadHelper for SampleTableBox {
    /// Calculates the end range of the SampleTableBox, considering all sub-boxes.
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size() // Return the end position after considering total size
    }

    /// Calculates the total size of the SampleTableBox in bytes, including the BoxHeader and all sub-boxes.
    fn total_size(&self) -> usize {
        let header_size = self.header.total_size(); // Size of BoxHeader (fixed part)
        let stsd_size = self.stsd.total_size(); // Size of SampleDescriptionBox
        let stts_size = self.stts.total_size(); // Size of TimeToSampleBox
        let stsc_size = self.stsc.total_size(); // Size of SampleToChunkBox
        let stsz_size = self.stsz.total_size(); // Size of SampleSizeBox
        let stco_size = self.stco.total_size(); // Size of ChunkOffsetBox

        header_size + stsd_size + stts_size + stsc_size + stsz_size + stco_size // Total size
    }
}
