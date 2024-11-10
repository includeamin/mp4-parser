use crate::boxes::{ftyp::Ftyp, moov::MovieBox};

#[derive(Debug)]
pub struct MP4 {
    pub ftyp: Option<Ftyp>,
    pub moov: Option<MovieBox>,
}

impl MP4 {
    pub fn new(ftyp: Ftyp, moov: MovieBox) -> Self {
        Self {
            ftyp: Some(ftyp),
            moov: Some(moov),
        }
    }
}

impl Default for MP4 {
    fn default() -> Self {
        Self {
            ftyp: None,
            moov: None,
        }
    }
}
