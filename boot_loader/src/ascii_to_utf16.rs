pub fn ascii_to_utf16<T: Iterator<Item = u8>>(iter: T) -> impl Iterator<Item = u16> {
    iter.map(|v| v as u16)
}
