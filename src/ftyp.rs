use crate::{header::BoxHeader, shared_consts::CHUNK_SIZE, utils::get_range};

pub const HEADER_FTYP: &str = "ftyp";
const FTYP_MAJOR_BRAND: std::ops::Range<usize> = 8..12;
const FTYP_MINOR_VERSION: std::ops::Range<usize> = 12..16;
const FTYP_COMAPTIBLE_BRANDS: std::ops::Range<usize> = 16..32;

#[derive(Debug)]
pub struct FtypBox {
    pub header: BoxHeader,
    pub major_brand: String,
    pub minor_version: u32,
    pub compatible_brands: Vec<String>,
}

impl FtypBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        if header.name != HEADER_FTYP {
            panic!("invalid header")
        }

        let mut compatible_brans: Vec<String> = Vec::new();
        for i in buffer[get_range(seek, FTYP_COMAPTIBLE_BRANDS)].chunks(CHUNK_SIZE) {
            compatible_brans.push(String::from_utf8(i.into()).unwrap());
        }

        Self {
            header,
            major_brand: String::from_utf8(
                buffer[get_range(seek, FTYP_MAJOR_BRAND)]
                    .into(),
            )
            .unwrap(),
            minor_version: u32::from_be_bytes(
                buffer[get_range(seek, FTYP_MINOR_VERSION)]
                    .try_into()
                    .unwrap(),
            ),
            compatible_brands: compatible_brans,
        }
    }

    pub fn get_end_range(seek: usize) -> usize {
        seek + FTYP_COMAPTIBLE_BRANDS.end
    }
}
