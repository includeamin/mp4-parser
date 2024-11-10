use crate::utils::{get_range, ReadHelper};

use super::header::BoxHeader;

// Constants for MediaHeaderBox
const MEDIA_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9; // 1 byte
const MEDIA_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12; // 3 bytes
const MEDIA_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16; // 4 bytes
const MEDIA_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20; // 4 bytes
const MEDIA_HEADER_BOX_TIMESCALE: std::ops::Range<usize> = 20..24; // 4 bytes
const MEDIA_HEADER_BOX_DURATION: std::ops::Range<usize> = 24..28; // 4 bytes

// Constants for field sizes
const VERSION_SIZE: usize = 1; // 1 byte
const FLAGS_SIZE: usize = 3; // 3 bytes
const CREATION_TIME_SIZE: usize = 4; // 4 bytes
const MODIFICATION_TIME_SIZE: usize = 4; // 4 bytes
const TIMESCALE_SIZE: usize = 4; // 4 bytes
const DURATION_SIZE: usize = 4; // 4 bytes

#[derive(Debug, Clone)]
pub struct MediaHeaderBox {
    header: BoxHeader,      // Size and type at offset 0–7
    version: u8,            // 1 byte at offset 8
    flags: [u8; 3],         // 3 bytes at offset 9–11
    creation_time: u32,     // 4 bytes at offset 12–15
    modification_time: u32, // 4 bytes at offset 16–19
    timescale: u32,         // 4 bytes at offset 20–23
    duration: u32,          // 4 bytes at offset 24–27
}

impl MediaHeaderBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let version = buffer[MEDIA_HEADER_BOX_VERSION][0];
        let flags = [
            buffer[MEDIA_HEADER_BOX_FLAGS][0],
            buffer[MEDIA_HEADER_BOX_FLAGS][1],
            buffer[MEDIA_HEADER_BOX_FLAGS][2],
        ];
        let creation_time =
            u32::from_be_bytes(buffer[MEDIA_HEADER_BOX_CREATION_TIME].try_into().unwrap());
        let modification_time = u32::from_be_bytes(
            buffer[MEDIA_HEADER_BOX_MODIFICATION_TIME]
                .try_into()
                .unwrap(),
        );
        let timescale = u32::from_be_bytes(buffer[MEDIA_HEADER_BOX_TIMESCALE].try_into().unwrap());
        let duration = u32::from_be_bytes(buffer[MEDIA_HEADER_BOX_DURATION].try_into().unwrap());

        MediaHeaderBox {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
        }
    }

    // Getter for `version`
    pub fn get_version(&self) -> u8 {
        self.version
    }

    // Getter for `flags`
    pub fn get_flags(&self) -> [u8; 3] {
        self.flags
    }

    // Getter for `creation_time`
    pub fn get_creation_time(&self) -> u32 {
        self.creation_time
    }

    // Getter for `modification_time`
    pub fn get_modification_time(&self) -> u32 {
        self.modification_time
    }

    // Getter for `timescale`
    pub fn get_timescale(&self) -> u32 {
        self.timescale
    }

    // Getter for `duration`
    pub fn get_duration(&self) -> u32 {
        self.duration
    }

    // Getter for `header`
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }
}

// Implementing ReadHelper trait for MediaHeaderBox
impl ReadHelper for MediaHeaderBox {
    /// Calculates the end range of the MediaHeaderBox, considering the header and data fields.
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size()
    }

    /// Calculates the total size of the MediaHeaderBox in bytes, including the BoxHeader and MediaHeaderBox fields.
    fn total_size(&self) -> usize {
        let header_size = self.header.total_size(); // Size of the BoxHeader
        let media_header_size = VERSION_SIZE
            + FLAGS_SIZE
            + CREATION_TIME_SIZE
            + MODIFICATION_TIME_SIZE
            + TIMESCALE_SIZE
            + DURATION_SIZE; // Fixed size fields of MediaHeaderBox

        header_size + media_header_size
    }
}
