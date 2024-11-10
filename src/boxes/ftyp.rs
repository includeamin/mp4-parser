use super::header::BoxHeader;
use crate::{shared_consts::CHUNK_SIZE, utils::ReadHelper};

pub const HEADER_FTYP: &str = "ftyp";

const FTYP_MAJOR_BRAND: std::ops::Range<usize> = 8..12;
const FTYP_MINOR_VERSION: std::ops::Range<usize> = 12..16;
const FTYP_COMAPTIBLE_BRANDS: std::ops::RangeFrom<usize> = 16..;

// Constants for sizes
const SIZE_MAJOR_BRAND: usize = 4;
const SIZE_MINOR_VERSION: usize = 4;
const SIZE_COMPATIBLE_BRAND_ENTRY: usize = 4;

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
        let compatible_version = buffer[FTYP_COMAPTIBLE_BRANDS.start..header.get_size() as usize]
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

    pub fn compatible_brands(&self) -> Vec<String> {
        let mut compatible_brands = Vec::new();
        for i in self.compatible_brands.clone() {
            compatible_brands.push(String::from_utf8(i.into()).unwrap());
        }

        compatible_brands
    }
}

/// Implementation of `ReadHelper` trait for `Ftyp` to calculate the end index of the box and total size.
impl ReadHelper for Ftyp {
    fn get_end_range(&self, seek: usize) -> usize {
        // Use the `total_size` method to calculate the end index of the box
        seek + self.total_size()
    }

    fn total_size(&self) -> usize {
        // Size of the header (BoxHeader)
        let header_size = self.header.total_size();

        // Size of compatible brands (each entry is 4 bytes)
        let compatible_brands_size = self.compatible_brands.len() * SIZE_COMPATIBLE_BRAND_ENTRY;

        // Total size is the sum of all these components
        header_size + SIZE_MAJOR_BRAND + SIZE_MINOR_VERSION + compatible_brands_size
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
    fn test_ftyp_get_end_range() {
        // Create an Ftyp instance from the buffer starting at seek position 4
        let ftyp = Ftyp::from_buffer(BUFFER);

        // Verify the total size of the ftyp box
        let expected_end_range = 4 + ftyp.total_size();
        assert_eq!(ftyp.get_end_range(4), expected_end_range);
    }

    #[test]
    fn test_ftyp_total_size() {
        // Create an Ftyp instance from the buffer starting at seek position 4
        let ftyp = Ftyp::from_buffer(BUFFER);

        // The total size should be the sum of the header size, major brand size,
        // minor version size, and compatible brands size.
        let expected_total_size = ftyp.header.total_size()
            + SIZE_MAJOR_BRAND
            + SIZE_MINOR_VERSION
            + (2 * SIZE_COMPATIBLE_BRAND_ENTRY); // Two compatible brands in the test buffer

        // Verify the total size
        assert_eq!(ftyp.total_size(), expected_total_size);
    }
}
