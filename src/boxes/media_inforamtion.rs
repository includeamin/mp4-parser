use super::header::BoxHeader;
use super::sample_table::SampleTableBox;

#[derive(Debug)]
pub struct MediaInformationBox {
    header: BoxHeader, // Size and type at offset 0–7
    stbl: SampleTableBox,
}

impl MediaInformationBox {
    pub fn from_buffer(seek: usize, buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(seek, buffer);
        let stbl = SampleTableBox::from_buffer(seek, buffer);

        MediaInformationBox { header, stbl }
    }

    // Getter for the header
    pub fn get_header(&self) -> &BoxHeader {
        &self.header
    }

    // Getter for the stbl (Sample Table Box)
    pub fn get_stbl(&self) -> &SampleTableBox {
        &self.stbl
    }
}
