use super::header::BoxHeader;
use crate::utils::{get_range, get_range_from};

const CHUNK_OFFSET_BOX_ENTRY_COUNT: std::ops::Range<usize> = 8..12;
const CHUNK_OFFSET_BOX_CHUNK_OFFSETS: std::ops::RangeFrom<usize> = 12..;

#[derive(Debug)]
pub struct ChunkOffsetBox {
    header: BoxHeader,       // Size and type at offset 0–7
    entry_count: u32,        // 4 bytes at offset 8–11
    chunk_offsets: Vec<u32>, // Variable length after offset 12
}

impl ChunkOffsetBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let entry_count = u32::from_be_bytes(
            buffer[get_range(seek, CHUNK_OFFSET_BOX_ENTRY_COUNT)]
                .try_into()
                .unwrap(),
        );

        // For chunk_offsets, it's variable-length, so we parse them.
        let chunk_offsets = buffer[get_range_from(seek, CHUNK_OFFSET_BOX_CHUNK_OFFSETS)]
            .chunks(4)
            .map(|chunk| u32::from_be_bytes(chunk.try_into().unwrap()))
            .collect::<Vec<_>>();

        ChunkOffsetBox {
            header,
            entry_count,
            chunk_offsets,
        }
    }

    // Getter for entry_count
    pub fn entry_count(&self) -> u32 {
        self.entry_count
    }

    // Getter for chunk_offsets
    pub fn chunk_offsets(&self) -> &Vec<u32> {
        &self.chunk_offsets
    }

    // Getter for header (if needed)
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }
}
