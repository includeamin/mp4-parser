use crate::{
    header::BoxHeader,
    shared_consts::CHUNK_SIZE,
    utils::{get_range, ReadHelper},
};

pub const HEADER_FTYP: &str = "ftyp";
const FTYP_MAJOR_BRAND: std::ops::Range<usize> = 8..12;
const FTYP_MINOR_VERSION: std::ops::Range<usize> = 12..16;
const FTYP_COMAPTIBLE_BRANDS: std::ops::Range<usize> = 16..32;

#[derive(Debug)]
pub struct FtypBox {
    header: BoxHeader,               // Size and type at offset 0–7
    major_brand: [u8; 4],            // 4 bytes at offset 8–11 (e.g., "isom", "mp42")
    minor_version: u32,              // 4 bytes at offset 12–15 (usually a version number)
    compatible_brands: Vec<[u8; 4]>, // Each entry is 4 bytes, starts at offset 16
}

impl FtypBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        if header.get_box_type() != HEADER_FTYP {
            panic!("invalid header")
        }

        let range = get_range(seek, FTYP_COMAPTIBLE_BRANDS);
        let slice = &buffer[range]; // This is a &[u8] slice
        let compatible_version = slice
            .chunks(CHUNK_SIZE)
            .filter_map(|chunk| {
                if chunk.len() == CHUNK_SIZE {
                    let mut arr = [0u8; CHUNK_SIZE];
                    arr.copy_from_slice(chunk); // Copy the chunk into the array
                    Some(arr)
                } else {
                    None // Ignore incomplete chunks that don't fit the full size
                }
            })
            .collect();

        Self {
            header,
            major_brand: buffer[get_range(seek, FTYP_MAJOR_BRAND)]
                .try_into()
                .unwrap(),
            minor_version: u32::from_be_bytes(
                buffer[get_range(seek, FTYP_MINOR_VERSION)]
                    .try_into()
                    .unwrap(),
            ),
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
        let mut compatible_brans = Vec::new();
        for i in self.compatible_brands.clone() {
            compatible_brans.push(String::from_utf8(i.try_into().unwrap()).unwrap());
        }

        compatible_brans
    }
}

impl ReadHelper for FtypBox {
    fn get_end_range(seek: usize) -> usize {
        seek + FTYP_COMAPTIBLE_BRANDS.end
    }
}