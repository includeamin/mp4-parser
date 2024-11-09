// use crate::header::{self, BoxHeader};

// use std::vec::Vec;

// pub const HEADER_MOOV: &str = "moov";

// #[derive(Debug)]
// struct MovieHeaderBox {
//     header: BoxHeader,      // Size and type at offset 0–7
//     version: u8,            // 1 byte at offset 8
//     flags: [u8; 3],         // 3 bytes at offset 9–11
//     creation_time: u32,     // 4 bytes at offset 12–15
//     modification_time: u32, // 4 bytes at offset 16–19
//     timescale: u32,         // 4 bytes at offset 20–23
//     duration: u32,          // 4 bytes at offset 24–27
//     rate: f32,              // 4 bytes at offset 28–31 (16.16 fixed-point)
//     volume: f32,            // 2 bytes at offset 32–33 (8.8 fixed-point)
//     reserved: [u8; 10],     // 10 bytes reserved at offset 34–43
//     matrix: [u32; 9],       // 36 bytes at offset 44–79
//     next_track_id: u32,     // 4 bytes at offset 80–83
// }

// #[derive(Debug)]
// struct TrackHeaderBox {
//     header: BoxHeader,      // Size and type at offset 0–7
//     version: u8,            // 1 byte at offset 8
//     flags: [u8; 3],         // 3 bytes at offset 9–11
//     creation_time: u32,     // 4 bytes at offset 12–15
//     modification_time: u32, // 4 bytes at offset 16–19
//     track_id: u32,          // 4 bytes at offset 20–23
//     reserved: u32,          // 4 bytes reserved at offset 24–27
//     duration: u32,          // 4 bytes at offset 28–31
//     reserved2: [u8; 8],     // 8 bytes reserved at offset 32–39
//     layer: u16,             // 2 bytes at offset 40–41
//     alternate_group: u16,   // 2 bytes at offset 42–43
//     volume: f32,            // 2 bytes at offset 44–45 (8.8 fixed-point, only for audio)
//     reserved3: u16,         // 2 bytes reserved at offset 46–47
//     matrix: [u32; 9],       // 36 bytes at offset 48–83
//     width: f32,             // 4 bytes at offset 84–87 (16.16 fixed-point)
//     height: f32,            // 4 bytes at offset 88–91 (16.16 fixed-point)
// }

// #[derive(Debug)]
// struct MediaHeaderBox {
//     header: BoxHeader,      // Size and type at offset 0–7
//     version: u8,            // 1 byte at offset 8
//     flags: [u8; 3],         // 3 bytes at offset 9–11
//     creation_time: u32,     // 4 bytes at offset 12–15
//     modification_time: u32, // 4 bytes at offset 16–19
//     timescale: u32,         // 4 bytes at offset 20–23
//     duration: u32,          // 4 bytes at offset 24–27
// }

// #[derive(Debug)]
// struct HandlerBox {
//     header: BoxHeader,     // Size and type at offset 0–7
//     version: u8,           // 1 byte at offset 8
//     flags: [u8; 3],        // 3 bytes at offset 9–11
//     handler_type: [u8; 4], // 4 bytes at offset 12–15 (e.g., 'vide' for video, 'soun' for audio)
//     reserved: [u8; 12],    // 12 bytes reserved at offset 16–27
//     name: Vec<u8>,         // Null-terminated string starting at offset 28
// }

// #[derive(Debug)]
// struct SampleDescriptionBox {
//     header: BoxHeader,           // Size and type at offset 0–7
//     sample_count: u32,           // 4 bytes at offset 8–11
//     sample_description: Vec<u8>, // Variable length after offset 12
// }

// #[derive(Debug)]
// struct TimeToSampleBox {
//     header: BoxHeader,        // Size and type at offset 0–7
//     entry_count: u32,         // 4 bytes at offset 8–11
//     entries: Vec<(u32, u32)>, // Variable length: each entry has sample count and duration
// }

// #[derive(Debug)]
// struct SampleToChunkBox {
//     header: BoxHeader,             // Size and type at offset 0–7
//     entry_count: u32,              // 4 bytes at offset 8–11
//     entries: Vec<(u32, u32, u32)>, // Variable length: first_chunk, samples_per_chunk, sample_description_index
// }

// #[derive(Debug)]
// struct SampleSizeBox {
//     header: BoxHeader,      // Size and type at offset 0–7
//     sample_count: u32,      // 4 bytes at offset 8–11
//     sample_sizes: Vec<u32>, // Variable length after offset 12
// }

// #[derive(Debug)]
// struct ChunkOffsetBox {
//     header: BoxHeader,       // Size and type at offset 0–7
//     entry_count: u32,        // 4 bytes at offset 8–11
//     chunk_offsets: Vec<u32>, // Variable length after offset 12
// }

// #[derive(Debug)]
// struct SampleTableBox {
//     header: BoxHeader, // Size and type at offset 0–7
//     stsd: SampleDescriptionBox,
//     stts: TimeToSampleBox,
//     stsc: SampleToChunkBox,
//     stsz: SampleSizeBox,
//     stco: ChunkOffsetBox,
// }

// #[derive(Debug)]
// struct MediaInformationBox {
//     header: BoxHeader, // Size and type at offset 0–7
//     stbl: SampleTableBox,
// }

// #[derive(Debug)]
// struct MediaBox {
//     header: BoxHeader, // Size and type at offset 0–7
//     mdhd: MediaHeaderBox,
//     hdlr: HandlerBox,
//     minf: MediaInformationBox,
// }

// #[derive(Debug)]
// struct TrackBox {
//     header: BoxHeader, // Size and type at offset 0–7
//     tkhd: TrackHeaderBox,
//     mdia: MediaBox,
// }

// #[derive(Debug)]
// struct MovieBox {
//     header: BoxHeader, // Size and type at offset 0–7
//     mvhd: MovieHeaderBox,
//     traks: Vec<TrackBox>, // List of Track Boxes
// }
