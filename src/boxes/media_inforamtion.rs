use crate::utils::ReadHelper;

use super::header::BoxHeader;
use super::sample_table::SampleTableBox;

const MEDIA_INFORMATION_BOX_HEADER_SIZE: usize = 8; // Size of BoxHeader (fixed)

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

// Implementing ReadHelper trait for MediaInformationBox
impl ReadHelper for MediaInformationBox {
    /// Calculates the end range of the MediaInformationBox, considering the header and stbl fields.
    fn get_end_range(&self, seek: usize) -> usize {
        seek + self.total_size() // Return the end position after considering total size
    }

    /// Calculates the total size of the MediaInformationBox in bytes, including the BoxHeader and SampleTableBox fields.
    fn total_size(&self) -> usize {
        let header_size = self.header.total_size() as usize;
        let stbl_size = self.stbl.total_size(); // Size of SampleTableBox

        header_size + stbl_size // Total size is the sum of both components
    }
}
