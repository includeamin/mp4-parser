pub fn get_range(seek: usize, range: std::ops::Range<usize>) -> std::ops::Range<usize> {
    seek + range.start..seek + range.end
}

pub fn get_range_from(
    seek: usize,
    range: std::ops::RangeFrom<usize>,
) -> std::ops::RangeFrom<usize> {
    seek + range.start..
}

pub trait ReadHelper {
    fn get_end_range(seek: usize) -> usize;
}
