use crate::utils::get_range;

use super::header::BoxHeader;

// consts for TrackHeaderBox
const TRACK_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9; // 1 byte
const TRACK_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12; // 3 bytes
const TRACK_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16; // 4 bytes
const TRACK_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20; // 4 bytes
const TRACK_HEADER_BOX_TRACK_ID: std::ops::Range<usize> = 20..24; // 4 bytes
const TRACK_HEADER_BOX_RESERVED: std::ops::Range<usize> = 24..28; // 4 bytes
const TRACK_HEADER_BOX_DURATION: std::ops::Range<usize> = 28..32; // 4 bytes
const TRACK_HEADER_BOX_RESERVED2: std::ops::Range<usize> = 32..40; // 8 bytes
const TRACK_HEADER_BOX_LAYER: std::ops::Range<usize> = 40..42; // 2 bytes
const TRACK_HEADER_BOX_ALTERNATE_GROUP: std::ops::Range<usize> = 42..44; // 2 bytes
const TRACK_HEADER_BOX_VOLUME: std::ops::Range<usize> = 44..46; // 2 bytes (8.8 fixed-point)
const TRACK_HEADER_BOX_RESERVED3: std::ops::Range<usize> = 46..48; // 2 bytes
const TRACK_HEADER_BOX_MATRIX: std::ops::Range<usize> = 48..84; // 36 bytes (9 x 4 bytes)
const TRACK_HEADER_BOX_WIDTH: std::ops::Range<usize> = 84..88; // 4 bytes (16.16 fixed-point)
const TRACK_HEADER_BOX_HEIGHT: std::ops::Range<usize> = 88..92; // 4 bytes (16.16 fixed-point)

#[derive(Debug)]
pub struct TrackHeaderBox {
    header: BoxHeader,      // Size and type at offset 0–7
    version: u8,            // 1 byte at offset 8
    flags: [u8; 3],         // 3 bytes at offset 9–11
    creation_time: u32,     // 4 bytes at offset 12–15
    modification_time: u32, // 4 bytes at offset 16–19
    track_id: u32,          // 4 bytes at offset 20–23
    reserved: u32,          // 4 bytes reserved at offset 24–27
    duration: u32,          // 4 bytes at offset 28–31
    reserved2: [u8; 8],     // 8 bytes reserved at offset 32–39
    layer: u16,             // 2 bytes at offset 40–41
    alternate_group: u16,   // 2 bytes at offset 42–43
    volume: f32,            // 2 bytes at offset 44–45 (8.8 fixed-point, only for audio)
    reserved3: u16,         // 2 bytes reserved at offset 46–47
    matrix: [u32; 9],       // 36 bytes at offset 48–83
    width: f32,             // 4 bytes at offset 84–87 (16.16 fixed-point)
    height: f32,            // 4 bytes at offset 88–91 (16.16 fixed-point)
}

impl TrackHeaderBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);

        let version = buffer[get_range(seek, TRACK_HEADER_BOX_VERSION)][0];

        let flags = [
            buffer[get_range(seek, TRACK_HEADER_BOX_FLAGS)][0],
            buffer[get_range(seek, TRACK_HEADER_BOX_FLAGS)][1],
            buffer[get_range(seek, TRACK_HEADER_BOX_FLAGS)][2],
        ];

        let creation_time = u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_CREATION_TIME)]
                .try_into()
                .unwrap(),
        );

        let modification_time = u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_MODIFICATION_TIME)]
                .try_into()
                .unwrap(),
        );

        let track_id = u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_TRACK_ID)]
                .try_into()
                .unwrap(),
        );

        let reserved = u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_RESERVED)]
                .try_into()
                .unwrap(),
        );

        let duration = u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_DURATION)]
                .try_into()
                .unwrap(),
        );

        let mut reserved2 = [0u8; 8];
        reserved2.copy_from_slice(&buffer[get_range(seek, TRACK_HEADER_BOX_RESERVED2)]);

        let layer = u16::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_LAYER)]
                .try_into()
                .unwrap(),
        );

        let alternate_group = u16::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_ALTERNATE_GROUP)]
                .try_into()
                .unwrap(),
        );

        let volume = (buffer[get_range(seek, TRACK_HEADER_BOX_VOLUME)][0] as f32
            + (buffer[get_range(seek, TRACK_HEADER_BOX_VOLUME)][1] as f32 / 256.0));

        let reserved3 = u16::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_RESERVED3)]
                .try_into()
                .unwrap(),
        );

        let mut matrix = [0u32; 9];
        for (i, chunk) in matrix.iter_mut().enumerate() {
            *chunk = u32::from_be_bytes(
                buffer[get_range(
                    seek,
                    TRACK_HEADER_BOX_MATRIX.start + i * 4
                        ..TRACK_HEADER_BOX_MATRIX.start + i * 4 + 4,
                )]
                .try_into()
                .unwrap(),
            );
        }

        let width = f32::from_bits(u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_WIDTH)]
                .try_into()
                .unwrap(),
        ));

        let height = f32::from_bits(u32::from_be_bytes(
            buffer[get_range(seek, TRACK_HEADER_BOX_HEIGHT)]
                .try_into()
                .unwrap(),
        ));

        TrackHeaderBox {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            track_id,
            reserved,
            duration,
            reserved2,
            layer,
            alternate_group,
            volume,
            reserved3,
            matrix,
            width,
            height,
        }
    }
}
