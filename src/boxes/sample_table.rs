use super::header::BoxHeader;

use super::{
    chunk_offset::ChunkOffsetBox, sample_description::SampleDescriptionBox,
    sample_size::SampleSizeBox, sample_to_chunk::SampleToChunkBox, time_to_sample::TimeToSampleBox,
};

#[derive(Debug)]
pub struct SampleTableBox {
    header: BoxHeader, // Size and type at offset 0–7
    stsd: SampleDescriptionBox,
    stts: TimeToSampleBox,
    stsc: SampleToChunkBox,
    stsz: SampleSizeBox,
    stco: ChunkOffsetBox,
}

impl SampleTableBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let stsd = SampleDescriptionBox::from_buffer(seek, buffer);
        let stts = TimeToSampleBox::from_buffer(seek, buffer);
        let stsc = SampleToChunkBox::from_buffer(seek, buffer);
        let stsz = SampleSizeBox::from_buffer(seek, buffer);
        let stco = ChunkOffsetBox::from_buffer(seek, buffer);

        SampleTableBox {
            header,
            stsd,
            stts,
            stsc,
            stsz,
            stco,
        }
    }
}
