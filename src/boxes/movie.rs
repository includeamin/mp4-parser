use super::header::BoxHeader;
use super::{movie_header::MovieHeaderBox, track::TrackBox};
use crate::utils::get_range_from;

pub const HEADER_MOOV: &str = "moov";
const MOVIE_BOX_TRAKS: std::ops::RangeFrom<usize> = 12..; // Variable-length, depends on the box structure

/// The `MovieBox` struct represents the 'moov' box in an MP4 file, which contains information about
/// the movie, including metadata (`mvhd`) and a list of `TrackBox`es that make up the movie.
///
/// # Fields:
/// - `header`: Contains the size and type information for the `MovieBox` (`moov`).
/// - `mvhd`: The `MovieHeaderBox` that contains the movie-level metadata (e.g., timescale, duration).
/// - `traks`: A vector of `TrackBox`es, each representing a track in the movie (e.g., video, audio).
#[derive(Debug)]
pub struct MovieBox {
    header: BoxHeader,    // Size and type at offset 0–7
    mvhd: MovieHeaderBox, // Movie header box containing movie-level metadata
    traks: Vec<TrackBox>, // List of Track Boxes representing individual tracks in the movie
}

impl MovieBox {
    /// Parses a `MovieBox` from a buffer at the given seek position.
    ///
    /// # Parameters:
    /// - `seek`: The starting byte position in the buffer where the `MovieBox` begins.
    /// - `buffer`: The byte slice that contains the data for the `MovieBox`.
    ///
    /// # Returns:
    /// A `MovieBox` instance with the parsed data.
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let mvhd = MovieHeaderBox::from_buffer(seek, buffer);

        // Extract the traks (Track Boxes)
        let mut traks = Vec::new();
        let traks_range = get_range_from(seek, MOVIE_BOX_TRAKS);
        let mut trak_offset = traks_range.start;
        let end = mvhd.get_header().get_size() as usize;

        // Currently commented out code for processing TrackBoxes
        // loop {
        //     traks.push(TrackBox::from_buffer(trak_offset, buffer));
        //     trak_offset += traks.last().unwrap().header.size as usize;
        // }

        MovieBox {
            header,
            mvhd,
            traks,
        }
    }

    /// Returns a reference to the `BoxHeader` of the `MovieBox`.
    ///
    /// # Returns:
    /// A reference to the `BoxHeader`.
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    /// Returns a reference to the `MovieHeaderBox` of the `MovieBox`.
    ///
    /// # Returns:
    /// A reference to the `MovieHeaderBox`.
    pub fn movie_header(&self) -> &MovieHeaderBox {
        &self.mvhd
    }

    /// Returns a reference to the vector of `TrackBox`es in the `MovieBox`.
    ///
    /// # Returns:
    /// A reference to the vector of `TrackBox`es.
    pub fn tracks(&self) -> &Vec<TrackBox> {
        &self.traks
    }

    /// Returns the number of tracks in the `MovieBox`.
    ///
    /// # Returns:
    /// The number of tracks (`usize`).
    pub fn track_count(&self) -> usize {
        self.traks.len()
    }
}
