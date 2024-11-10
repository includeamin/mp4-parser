use super::header::BoxHeader;
use crate::utils::{get_range, ReadHelper};

// consts for MovieHeaderBox
const MOVIE_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9;
const MOVIE_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12;
const MOVIE_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16;
const MOVIE_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20;
const MOVIE_HEADER_BOX_TIMESCALE: std::ops::Range<usize> = 20..24;
const MOVIE_HEADER_BOX_DURATION: std::ops::Range<usize> = 24..28;
const MOVIE_HEADER_BOX_RATE: std::ops::Range<usize> = 28..32;
const MOVIE_HEADER_BOX_VOLUME: std::ops::Range<usize> = 32..34;
const MOVIE_HEADER_BOX_RESERVED: std::ops::Range<usize> = 34..44;
const MOVIE_HEADER_BOX_MATRIX: std::ops::Range<usize> = 44..80;
const MOVIE_HEADER_BOX_NEXT_TRACK_ID: std::ops::Range<usize> = 80..84;

#[derive(Debug, Clone)]
pub struct MovieHeaderBox {
    header: BoxHeader,      // Size and type at offset 0–7
    version: u8,            // 1 byte at offset 8
    flags: [u8; 3],         // 3 bytes at offset 9–11
    creation_time: u32,     // 4 bytes at offset 12–15
    modification_time: u32, // 4 bytes at offset 16–19
    timescale: u32,         // 4 bytes at offset 20–23
    duration: u32,          // 4 bytes at offset 24–27
    rate: f32,              // 4 bytes at offset 28–31 (16.16 fixed-point)
    volume: f32,            // 2 bytes at offset 32–33 (8.8 fixed-point)
    reserved: [u8; 10],     // 10 bytes reserved at offset 34–43
    matrix: [u32; 9],       // 36 bytes at offset 44–79
    next_track_id: u32,     // 4 bytes at offset 80–83
}

impl MovieHeaderBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let version = buffer[MOVIE_HEADER_BOX_VERSION][0];

        let flags = [
            buffer[MOVIE_HEADER_BOX_FLAGS][0],
            buffer[MOVIE_HEADER_BOX_FLAGS][1],
            buffer[MOVIE_HEADER_BOX_FLAGS][2],
        ];

        let creation_time =
            u32::from_be_bytes(buffer[MOVIE_HEADER_BOX_CREATION_TIME].try_into().unwrap());

        let modification_time = u32::from_be_bytes(
            buffer[MOVIE_HEADER_BOX_MODIFICATION_TIME]
                .try_into()
                .unwrap(),
        );

        let timescale = u32::from_be_bytes(buffer[MOVIE_HEADER_BOX_TIMESCALE].try_into().unwrap());

        let duration = u32::from_be_bytes(buffer[MOVIE_HEADER_BOX_DURATION].try_into().unwrap());

        let rate = f32::from_bits(u32::from_be_bytes(
            buffer[MOVIE_HEADER_BOX_RATE].try_into().unwrap(),
        ));

        let volume = buffer[MOVIE_HEADER_BOX_VOLUME][0] as f32
            + (buffer[MOVIE_HEADER_BOX_VOLUME][1] as f32 / 256.0);

        let mut reserved = [0u8; 10];
        reserved.copy_from_slice(&buffer[MOVIE_HEADER_BOX_RESERVED]);

        let mut matrix = [0u32; 9];
        for (i, chunk) in matrix.iter_mut().enumerate() {
            *chunk = u32::from_be_bytes(
                buffer[MOVIE_HEADER_BOX_MATRIX.start + i * 4
                    ..MOVIE_HEADER_BOX_MATRIX.start + i * 4 + 4]
                    .try_into()
                    .unwrap(),
            );
        }

        let next_track_id =
            u32::from_be_bytes(buffer[MOVIE_HEADER_BOX_NEXT_TRACK_ID].try_into().unwrap());

        MovieHeaderBox {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            rate,
            volume,
            reserved,
            matrix,
            next_track_id,
        }
    }

    // Getter for the header
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }

    // Getter for the version
    pub fn get_version(&self) -> u8 {
        self.version
    }

    // Getter for the flags
    pub fn get_flags(&self) -> &[u8; 3] {
        &self.flags
    }

    // Getter for the creation time
    pub fn get_creation_time(&self) -> u32 {
        self.creation_time
    }

    // Getter for the modification time
    pub fn get_modification_time(&self) -> u32 {
        self.modification_time
    }

    // Getter for the timescale
    pub fn get_timescale(&self) -> u32 {
        self.timescale
    }

    // Getter for the duration
    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    // Getter for the rate
    pub fn get_rate(&self) -> f32 {
        self.rate
    }

    // Getter for the volume
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    // Getter for the reserved field
    pub fn get_reserved(&self) -> &[u8; 10] {
        &self.reserved
    }

    // Getter for the matrix
    pub fn get_matrix(&self) -> &[u32; 9] {
        &self.matrix
    }

    // Getter for the next track ID
    pub fn get_next_track_id(&self) -> u32 {
        self.next_track_id
    }
}

impl ReadHelper for MovieHeaderBox {
    /// Returns the end byte position of this header based on its starting position.
    ///
    /// # Parameters
    /// - `seek`: The starting byte position of this `MovieHeaderBox`.
    ///
    /// # Returns
    /// The end byte position (inclusive) of the `MovieHeaderBox`.
    fn get_end_range(&self, seek: usize) -> usize {
        // seek + self.total_size() - 1 // inclusive last byte
        seek + self.total_size()
    }

    /// Returns the total size of this `MovieHeaderBox` in bytes, including the `BoxHeader` and all fields.
    ///
    /// # Returns
    /// The total byte size of the `MovieHeaderBox`.
    fn total_size(&self) -> usize {
        // Add up the byte ranges defined for `MovieHeaderBox` to calculate the total size.
        MOVIE_HEADER_BOX_NEXT_TRACK_ID.end
    }
}
