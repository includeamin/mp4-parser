use super::header::BoxHeader;
use std::convert::TryInto;

// Constants for MediaHeaderBox
const MEDIA_HEADER_BOX_VERSION: std::ops::Range<usize> = 8..9; // 1 byte
const MEDIA_HEADER_BOX_FLAGS: std::ops::Range<usize> = 9..12; // 3 bytes
const MEDIA_HEADER_BOX_CREATION_TIME: std::ops::Range<usize> = 12..16; // 4 bytes
const MEDIA_HEADER_BOX_MODIFICATION_TIME: std::ops::Range<usize> = 16..20; // 4 bytes
const MEDIA_HEADER_BOX_TIMESCALE: std::ops::Range<usize> = 20..24; // 4 bytes
const MEDIA_HEADER_BOX_DURATION: std::ops::Range<usize> = 24..28; // 4 bytes
const MEDIA_HEADER_BOX_LANGUAGE: std::ops::Range<usize> = 28..30; // 2 bytes (ISO-639-2 Language)

#[derive(Debug, Clone)]
pub struct MediaHeaderBox {
    header: BoxHeader,      // Size and type at offset 0–7
    version: u8,            // 1 byte at offset 8
    flags: [u8; 3],         // 3 bytes at offset 9–11
    creation_time: u32,     // 4 bytes at offset 12–15
    modification_time: u32, // 4 bytes at offset 16–19
    timescale: u32,         // 4 bytes at offset 20–23
    duration: u32,          // 4 bytes at offset 24–27
    language: [u8; 2],      // 2 bytes at offset 28–29 (ISO-639-2 Language code)
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
        let language = [
            buffer[MEDIA_HEADER_BOX_LANGUAGE][0],
            buffer[MEDIA_HEADER_BOX_LANGUAGE][1],
        ];

        MediaHeaderBox {
            header,
            version,
            flags,
            creation_time,
            modification_time,
            timescale,
            duration,
            language,
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

    // Getter for `language`
    pub fn get_language(&self) -> &str {
        // Convert language code to a string (ISO-639-2 format)
        match self.language {
            [0x55, 0xC4] => "und", // undetermined
            [0x6E, 0x61] => "eng", // English
            [0x66, 0x72] => "fra", // French
            [0x6A, 0x70] => "jpn", // Japanese
            _ => "unknown",        // Unknown language
        }
    }

    // Getter for `header`
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // Test the `from_buffer` function and getter methods for `MediaHeaderBox`
    #[test]
    fn test_media_header_box_from_buffer() {
        // Create a mock buffer that matches the structure of a MediaHeaderBox
        let mock_buffer: &[u8] = &[
            // Mock BoxHeader data (8 bytes)
            0x00, 0x00, 0x00, 0x14, // Box size
            0x6D, 0x64, 0x68, 0x64, // Box type "mdhd"
            // Version and flags (1 byte + 3 bytes)
            0x01, // Version
            0x00, 0x00, 0x00, // Flags
            // Creation time (4 bytes)
            0x00, 0x00, 0x00, 0x01, // Creation time
            // Modification time (4 bytes)
            0x00, 0x00, 0x00, 0x02, // Modification time
            // Timescale (4 bytes)
            0x00, 0x00, 0x00, 0x03, // Timescale
            // Duration (4 bytes)
            0x00, 0x00, 0x00, 0x04, // Duration
            // Language (2 bytes, ISO-639-2 format)
            0x6E, 0x61, // "eng" (English)
        ];

        // Call the `from_buffer` method to parse the buffer
        let media_header_box = MediaHeaderBox::from_buffer(mock_buffer);

        // Assert the parsed values
        assert_eq!(media_header_box.get_version(), 0x01); // Version should be 1
        assert_eq!(media_header_box.get_flags(), [0x00, 0x00, 0x00]); // Flags should be 0, 0, 0
        assert_eq!(media_header_box.get_creation_time(), 1); // Creation time should be 1
        assert_eq!(media_header_box.get_modification_time(), 2); // Modification time should be 2
        assert_eq!(media_header_box.get_timescale(), 3); // Timescale should be 3
        assert_eq!(media_header_box.get_duration(), 4); // Duration should be 4
        assert_eq!(media_header_box.get_language(), "eng"); // Language should be "eng" (English)
    }

    #[test]
    fn test_media_header_box_invalid_language() {
        // Create a mock buffer with an invalid language code
        let mock_buffer: &[u8] = &[
            // BoxHeader data
            0x00, 0x00, 0x00, 0x14, // Box size
            0x6D, 0x64, 0x68, 0x64, // Box type "mdhd"
            // Version and flags
            0x01, // Version
            0x00, 0x00, 0x00, // Flags
            // Creation time
            0x00, 0x00, 0x00, 0x01, // Creation time
            // Modification time
            0x00, 0x00, 0x00, 0x02, // Modification time
            // Timescale
            0x00, 0x00, 0x00, 0x03, // Timescale
            // Duration
            0x00, 0x00, 0x00, 0x04, // Duration
            // Invalid Language (not a recognized code)
            0x12, 0x34, // Invalid language
        ];

        // Call the `from_buffer` method to parse the buffer
        let media_header_box = MediaHeaderBox::from_buffer(mock_buffer);

        // Assert that the language should be "unknown"
        assert_eq!(media_header_box.get_language(), "unknown"); // Should return "unknown"
    }

    // Test the getters for BoxHeader (assuming BoxHeader has similar tests)
    #[test]
    fn test_box_header_getter() {
        let mock_buffer: &[u8] = &[
            0x00, 0x00, 0x00, 0x14, // Box size
            0x6D, 0x64, 0x68, 0x64, // Box type "mdhd"
        ];

        let box_header = BoxHeader::from_buffer(mock_buffer);

        // BoxHeader assertions
        assert_eq!(box_header.size(), 0x14); // Size should be 20 bytes
        assert_eq!(box_header.box_type(), "mdhd"); // Type should be "mdhd"
    }
}
