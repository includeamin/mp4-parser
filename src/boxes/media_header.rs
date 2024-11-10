use super::header::BoxHeader;

// Constants for MediaHeaderBox
const MEDIA_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9; // 1 byte
const MEDIA_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12; // 3 bytes
const MEDIA_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16; // 4 bytes
const MEDIA_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20; // 4 bytes
const MEDIA_HEADER_BOX_TIMESCALE: std::ops::Range<usize> = 20..24; // 4 bytes
const MEDIA_HEADER_BOX_DURATION: std::ops::Range<usize> = 24..28; // 4 bytes

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_header_box_from_buffer() {
        let buffer: &[u8] = &[
            // Mock BoxHeader for MediaHeaderBox (8 bytes)
            0x00, 0x00, 0x00, 0x20, // size = 32 bytes
            b'm', b'd', b'h', b'd', // type = "mdhd"
            // Version and flags
            0x01, // version = 1
            0x00, 0x00, 0x03, // flags = [0, 0, 3]
            // Creation time
            0x00, 0x00, 0x00, 0x10, // creation_time = 16
            // Modification time
            0x00, 0x00, 0x00, 0x20, // modification_time = 32
            // Timescale
            0x00, 0x00, 0x03, 0xE8, // timescale = 1000
            // Duration
            0x00, 0x00, 0x07, 0xD0, // duration = 2000
        ];

        let mdhd_box = MediaHeaderBox::from_buffer(buffer);

        // Test BoxHeader
        assert_eq!(mdhd_box.get_header().box_type(), "mdhd");
        assert_eq!(mdhd_box.get_header().size(), 32);

        // Test MediaHeaderBox fields
        assert_eq!(mdhd_box.get_version(), 1);
        assert_eq!(mdhd_box.get_flags(), [0, 0, 3]);
        assert_eq!(mdhd_box.get_creation_time(), 16);
        assert_eq!(mdhd_box.get_modification_time(), 32);
        assert_eq!(mdhd_box.get_timescale(), 1000);
        assert_eq!(mdhd_box.get_duration(), 2000);

        // Test total size calculation
        assert_eq!(mdhd_box.get_header().size(), 32);
    }
}
