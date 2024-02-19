use common::uefi::data_type::basic_type::{UnsignedInt16, UnsignedInt8};

pub fn ascii_to_utf16<T: Iterator<Item = UnsignedInt8>>(
    iter: T,
) -> impl Iterator<Item = UnsignedInt16> {
    iter.map(|v| v as UnsignedInt16)
}
