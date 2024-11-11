#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_parse_mp4() {
        let file = fs::read("tests/testdata/input.mp4").unwrap();
        let mp4 = mp4_parser::MP4Parser::from_buf(&file);

        let ftyp = mp4.ftyp.unwrap();
        assert_eq!(ftyp.major_brand(), "mp42");
        assert_eq!(ftyp.minor_version(), 0);
        assert_eq!(
            ftyp.compatible_brands(),
            vec!["mp42", "mp41", "isom", "avc1"]
        );

        println!("{:?}", mp4.moov.unwrap().movie_header())
    }
}
