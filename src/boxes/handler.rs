use crate::utils::{get_range, get_range_from};

use super::header::BoxHeader;

// consts for HandlerBox
const HANDLER_BOX_VERSION: std::ops::Range<usize> = 8..9;
const HANDLER_BOX_FLAGS: std::ops::Range<usize> = 9..12;
const HANDLER_BOX_HANDLER_TYPE: std::ops::Range<usize> = 12..16;
const HANDLER_BOX_RESERVED: std::ops::Range<usize> = 16..28;
const HANDLER_BOX_NAME_START: std::ops::RangeFrom<usize> = 28..; // Null-terminated, variable length

#[derive(Debug)]
pub struct HandlerBox {
    header: BoxHeader,     // Size and type at offset 0–7
    version: u8,           // 1 byte at offset 8
    flags: [u8; 3],        // 3 bytes at offset 9–11
    handler_type: [u8; 4], // 4 bytes at offset 12–15 (e.g., 'vide' for video, 'soun' for audio)
    reserved: [u8; 12],    // 12 bytes reserved at offset 16–27
    name: Vec<u8>,         // Null-terminated string starting at offset 28
}

impl HandlerBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let version = buffer[get_range(seek, HANDLER_BOX_VERSION)][0];
        let flags = [
            buffer[get_range(seek, HANDLER_BOX_FLAGS)][0],
            buffer[get_range(seek, HANDLER_BOX_FLAGS)][1],
            buffer[get_range(seek, HANDLER_BOX_FLAGS)][2],
        ];
        let handler_type = buffer[get_range(seek, HANDLER_BOX_HANDLER_TYPE)]
            .try_into()
            .unwrap();
        let reserved = buffer[get_range(seek, HANDLER_BOX_RESERVED)]
            .try_into()
            .unwrap();

        // For `name`, find the null-terminated string starting from `HANDLER_BOX_NAME_START`
        let name_start = get_range_from(seek, HANDLER_BOX_NAME_START).start;
        let name_end = buffer[name_start..]
            .iter()
            .position(|&b| b == 0)
            .map(|pos| name_start + pos)
            .unwrap_or(buffer.len());
        let name = buffer[name_start..name_end].to_vec();

        HandlerBox {
            header,
            version,
            flags,
            handler_type,
            reserved,
            name,
        }
    }
}
