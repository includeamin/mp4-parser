use crate::header;

pub const HEADER_MOOV: &str = "moov";

struct MovieHeader {}

struct MooVBox {
    header: header::BoxHeader,
    mvhd: MovieHeader,
}
