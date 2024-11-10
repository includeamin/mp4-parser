use super::header::BoxHeader;
use crate::shared_consts::CHUNK_SIZE;

pub const HEADER_FTYP: &str = "ftyp";

const FTYP_MAJOR_BRAND: std::ops::Range<usize> = 8..12;
const FTYP_MINOR_VERSION: std::ops::Range<usize> = 12..16;
const FTYP_COMAPTIBLE_BRANDS: std::ops::RangeFrom<usize> = 16..;

#[derive(Debug)]
pub struct Ftyp {
    header: BoxHeader,               // Size and type at offset 0–7
    major_brand: [u8; 4],            // 4 bytes at offset 8–11 (e.g., "isom", "mp42")
    minor_version: u32,              // 4 bytes at offset 12–15 (usually a version number)
    compatible_brands: Vec<[u8; 4]>, // Each entry is 4 bytes, starts at offset 16
}

impl Ftyp {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let compatible_version = buffer[FTYP_COMAPTIBLE_BRANDS.start..header.size()]
            .chunks(CHUNK_SIZE)
            .filter_map(|chunk| {
                if !chunk.is_empty() {
                    let mut arr = [0u8; CHUNK_SIZE];
                    arr[..chunk.len()].copy_from_slice(chunk); // Copy only the valid part of the chunk
                    Some(arr)
                } else {
                    None // Skip empty chunks (should not happen, but it's a safety check)
                }
            })
            .collect::<Vec<[u8; CHUNK_SIZE]>>();

        Self {
            header,
            major_brand: buffer[FTYP_MAJOR_BRAND].try_into().unwrap(),
            minor_version: u32::from_be_bytes(buffer[FTYP_MINOR_VERSION].try_into().unwrap()),
            compatible_brands: compatible_version,
        }
    }

    pub fn major_brand(&self) -> String {
        String::from_utf8(self.major_brand.to_vec()).unwrap()
    }

    pub fn minor_version(&self) -> u32 {
        self.minor_version
    }

    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    pub fn compatible_brands(&self) -> Vec<String> {
        let mut compatible_brands = Vec::new();
        for i in self.compatible_brands.clone() {
            compatible_brands.push(String::from_utf8(i.into()).unwrap());
        }

        compatible_brands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BUFFER: &[u8] = &[
        0x00, 0x00, 0x00, 0x18, // Size field (4 bytes): 24 bytes total size
        0x66, 0x74, 0x79, 0x70, // Type field ("ftyp")
        0x69, 0x73, 0x6F, 0x6D, // Major Brand ("isom")
        0x00, 0x00, 0x00, 0x01, // Minor Version (1)
        0x6D, 0x70, 0x34, 0x32, // Compatible Brand ("mp42")
        0x69, 0x73, 0x6F, 0x6D, // Compatible Brand ("isom")
        0x00, 0x01, 0x02, 0x03, // Random data after the box
        0x00, 0x01, 0x02, 0x03, 0x00, 0x01, 0x02, 0x03,
    ];

    #[test]
    fn test_ftyp_from_buffer() {
        // Create an Ftyp instance from the buffer starting at seek position 4
        let ftyp = Ftyp::from_buffer(BUFFER);

        // Verify the major brand is "isom"
        assert_eq!(ftyp.major_brand(), "isom");

        // Verify the minor version is 1
        assert_eq!(ftyp.minor_version(), 1);

        // Verify the compatible brands are "mp42" and "isom"
        let compatible_brands = ftyp.compatible_brands();
        assert_eq!(compatible_brands, vec!["mp42", "isom"]);
    }

    #[test]
    fn test_ftyp_total_size() {
        // Create an Ftyp instance from the buffer starting at seek position 4
        let ftyp = Ftyp::from_buffer(BUFFER);

        // Verify the total size
        assert_eq!(ftyp.header().size(), 24);
    }
}
