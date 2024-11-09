use crate::boxes::ftyp::Ftyp;

#[derive(Debug)]
pub struct MP4 {
    pub ftyp: Option<Ftyp>,
}

impl MP4 {
    pub fn new(ftyp: Ftyp) -> Self {
        Self { ftyp: Some(ftyp) }
    }
}

impl Default for MP4 {
    fn default() -> Self {
        Self { ftyp: None }
    }
}
