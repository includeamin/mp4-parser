#[cfg(test)]
mod tests {
    use std::fs;

    #[test]
    fn test_parse_mp4() {
        let file = fs::read("tests/testdata/input.mp4").unwrap();
        let mp4 = mp4_parser::MP4Parser::from_buf(&file);
        println!("{:?}", mp4);
        assert_eq!(mp4.ftyp.unwrap().major_brand(), "mp42")
    }
}
