use crate::utils::get_range;

use super::header::BoxHeader;
// consts for MediaHeaderBox
const MEDIA_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9; // 1 byte
const MEDIA_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12; // 3 bytes
const MEDIA_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16; // 4 bytes
const MEDIA_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20; // 4 bytes
const MEDIA_HEADER_BOX_TIMESCALE: std::ops::Range<usize> = 20..24; // 4 bytes
const MEDIA_HEADER_BOX_DURATION: std::ops::Range<usize> = 24..28; // 4 bytes

#[derive(Debug)]
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
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let version = buffer[get_range(seek, MEDIA_HEADER_BOX_VERSION)][0];
        let flags = [
            buffer[get_range(seek, MEDIA_HEADER_BOX_FLAGS)][0],
            buffer[get_range(seek, MEDIA_HEADER_BOX_FLAGS)][1],
            buffer[get_range(seek, MEDIA_HEADER_BOX_FLAGS)][2],
        ];
        let creation_time = u32::from_be_bytes(
            buffer[get_range(seek, MEDIA_HEADER_BOX_CREATION_TIME)]
                .try_into()
                .unwrap(),
        );
        let modification_time = u32::from_be_bytes(
            buffer[get_range(seek, MEDIA_HEADER_BOX_MODIFICATION_TIME)]
                .try_into()
                .unwrap(),
        );
        let timescale = u32::from_be_bytes(
            buffer[get_range(seek, MEDIA_HEADER_BOX_TIMESCALE)]
                .try_into()
                .unwrap(),
        );
        let duration = u32::from_be_bytes(
            buffer[get_range(seek, MEDIA_HEADER_BOX_DURATION)]
                .try_into()
                .unwrap(),
        );

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
}
