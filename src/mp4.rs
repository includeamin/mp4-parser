use crate::ftyp::FtypBox;

#[derive(Debug)]
pub struct MP4 {
    pub ftyp: Option<FtypBox>,
}

impl MP4 {
    pub fn new(ftyp: FtypBox) -> Self {
        Self { ftyp: Some(ftyp) }
    }
}

impl Default for MP4 {
    fn default() -> Self {
        Self { ftyp: None }
    }
}
