pub fn get_range(seek: usize, range: std::ops::Range<usize>) -> std::ops::Range<usize> {
    seek + range.start..seek + range.end
}
