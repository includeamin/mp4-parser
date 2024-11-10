use crate::utils::ReadHelper;

use super::header::BoxHeader;
use super::{media::MediaBox, track_header::TrackHeaderBox};

/// The `TrackBox` struct represents a track in an MP4 file, containing the track header (`tkhd`)
/// and media (`mdia`) data. It is one of the essential boxes that make up an MP4 file.
///
/// # Fields:
/// - `header`: Contains the size and type information for the `TrackBox`.
/// - `tkhd`: The `TrackHeaderBox` that contains metadata about the track (e.g., track ID, duration).
/// - `mdia`: The `MediaBox` that contains the media data for the track (e.g., sample data, media header).
#[derive(Debug, Clone)]
pub struct TrackBox {
    header: BoxHeader,    // Size and type at offset 0–7
    tkhd: TrackHeaderBox, // Track header box containing metadata
    mdia: MediaBox,       // Media box containing sample data and media header
}

impl TrackBox {
    /// Parses a `TrackBox` from a buffer at the given seek position.
    ///
    /// # Parameters:
    /// - `seek`: The starting byte position in the buffer where the `TrackBox` begins.
    /// - `buffer`: The byte slice that contains the data for the `TrackBox`.
    ///
    /// # Returns:
    /// A `TrackBox` instance with the parsed data.
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let tkhd = TrackHeaderBox::from_buffer(buffer);
        let mdia = MediaBox::from_buffer(buffer);

        TrackBox { header, tkhd, mdia }
    }

    /// Returns a reference to the `BoxHeader` of the `TrackBox`.
    ///
    /// # Returns:
    /// A reference to the `BoxHeader`.
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    /// Returns a reference to the `TrackHeaderBox` of the `TrackBox`.
    ///
    /// # Returns:
    /// A reference to the `TrackHeaderBox`.
    pub fn track_header(&self) -> &TrackHeaderBox {
        &self.tkhd
    }

    /// Returns a reference to the `MediaBox` of the `TrackBox`.
    ///
    /// # Returns:
    /// A reference to the `MediaBox`.
    pub fn media(&self) -> &MediaBox {
        &self.mdia
    }

    /// Returns the track ID from the `TrackHeaderBox`.
    ///
    /// # Returns:
    /// The track ID (`u32`).
    pub fn track_id(&self) -> u32 {
        self.tkhd.get_track_id()
    }

    /// Returns the duration of the track from the `TrackHeaderBox`.
    ///
    /// # Returns:
    /// The duration of the track in seconds (`u32`).
    pub fn duration(&self) -> u32 {
        self.tkhd.get_duration()
    }
}

impl ReadHelper for TrackBox {
    /// Returns the end range (the position of the last byte) based on the `seek` position.
    ///
    /// # Parameters:
    /// - `seek`: The starting byte position in the buffer where the `TrackBox` begins.
    ///
    /// # Returns:
    /// The end byte position (inclusive).
    fn get_end_range(&self, seek: usize) -> usize {
        // Calculate the total size by summing the sizes of the header, tkhd, and mdia
        let end_of_header = seek + self.header.total_size();
        let end_of_tkhd = end_of_header + self.tkhd.total_size();
        let end_of_mdia = end_of_tkhd + self.mdia.total_size();

        end_of_mdia - 1 // Return the last byte position, inclusive
    }

    /// Returns the total size of the `TrackBox` in bytes.
    ///
    /// # Returns:
    /// The total size of the `TrackBox`.
    fn total_size(&self) -> usize {
        self.header.total_size() + self.tkhd.total_size() + self.mdia.total_size()
    }
}
