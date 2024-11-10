use super::header::BoxHeader;
use super::sample_table::SampleTableBox;

#[derive(Debug, Clone)]
pub struct MediaInformationBox {
    header: BoxHeader, // Size and type at offset 0–7
    stbl: SampleTableBox,
}

impl MediaInformationBox {
    pub fn from_buffer(buffer: &[u8]) -> Self {
        let header = BoxHeader::from_buffer(buffer);
        let stbl = SampleTableBox::from_buffer(&buffer[8..]);

        MediaInformationBox { header, stbl }
    }

    // Getter for the header
    pub fn header(&self) -> &BoxHeader {
        &self.header
    }

    // Getter for the stbl (Sample Table Box)
    pub fn get_stbl(&self) -> &SampleTableBox {
        &self.stbl
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_media_information_box_from_buffer() {
        let buffer: &[u8] = &[
            // MediaInformationBox "minf"
            0x00, 0x00, 0x00, 0x60, // size = 96 bytes (total for "minf" box)
            b'm', b'i', b'n', b'f', // type = "minf"
            // SampleTableBox "stbl"
            0x00, 0x00, 0x00, 0x50, // size = 80 bytes (total for "stbl" box)
            b's', b't', b'b', b'l', // type = "stbl"
            // SampleDescriptionBox "stsd"
            0x00, 0x00, 0x00, 0x20, // size = 32 bytes
            b's', b't', b's', b'd', // type = "stsd"
            0x00, 0x00, 0x00, 0x01, // entry count (1 entry)
            0x00, 0x00, 0x00, 0x01, // sample description entry (mock data)
            // TimeToSampleBox "stts"
            0x00, 0x00, 0x00, 0x18, // size = 24 bytes
            b's', b't', b't', b's', // type = "stts"
            0x00, 0x00, 0x00, 0x01, // entry count (1 entry)
            0x00, 0x00, 0x00, 0x01, // sample count = 1
            0x00, 0x00, 0x00, 0x10, // duration = 16 (mock data)
            // SampleToChunkBox "stsc"
            0x00, 0x00, 0x00, 0x18, // size = 24 bytes
            b's', b't', b's', b'c', // type = "stsc"
            0x00, 0x00, 0x00, 0x01, // entry count (1 entry)
            0x00, 0x00, 0x00, 0x01, // first chunk (mock data)
            0x00, 0x00, 0x00, 0x01, // samples per chunk = 1 (mock data)
            // SampleSizeBox "stsz"
            0x00, 0x00, 0x00, 0x18, // size = 24 bytes
            b's', b't', b's', b'z', // type = "stsz"
            0x00, 0x00, 0x00, 0x01, // sample count = 1
            0x00, 0x00, 0x00, 0x10, // sample size = 16 bytes (mock data)
            // ChunkOffsetBox "stco"
            0x00, 0x00, 0x00, 0x18, // size = 24 bytes
            b's', b't', b'c', b'o', // type = "stco"
            0x00, 0x00, 0x00, 0x01, // entry count (1 entry)
            0x00, 0x00, 0x00, 0x20, // chunk offset = 32 (mock data)
        ];

        let minf_box = MediaInformationBox::from_buffer(buffer);

        // Test BoxHeader
        assert_eq!(minf_box.header().box_type(), "minf");
        assert_eq!(minf_box.header().size(), 48);

        // Test SampleTableBox (stbl)
        let stbl = minf_box.get_stbl();
        assert_eq!(stbl.get_header().box_type(), "stbl");
        assert_eq!(stbl.get_header().size(), 32);

        // Test SampleDescriptionBox (stsd)
        assert_eq!(
            stbl.get_sample_description().get_header().box_type(),
            "stsd"
        );
        assert_eq!(stbl.get_sample_description().get_header().size(), 16);

        // Test TimeToSampleBox (stts)
        assert_eq!(stbl.get_time_to_sample().get_header().box_type(), "stts");
        assert_eq!(stbl.get_time_to_sample().get_header().size(), 16);

        // Test SampleToChunkBox (stsc)
        assert_eq!(stbl.get_sample_to_chunk().get_header().box_type(), "stsc");
        assert_eq!(stbl.get_sample_to_chunk().get_header().size(), 16);

        // Test SampleSizeBox (stsz)
        assert_eq!(stbl.get_sample_size().get_header().box_type(), "stsz");
        assert_eq!(stbl.get_sample_size().get_header().size(), 16);

        // Test ChunkOffsetBox (stco)
        assert_eq!(stbl.get_chunk_offset().header().box_type(), "stco");
        assert_eq!(stbl.get_chunk_offset().header().size(), 16);

        // Test total size calculation
        let expected_total_size = minf_box.header().size() + stbl.get_header().size();
        assert_eq!(minf_box.header().size(), expected_total_size);
    }
}
