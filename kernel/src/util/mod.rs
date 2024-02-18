use common::uefi::data_type::basic_type::{UnsignedInt16, UnsignedInt32, UnsignedInt8};

pub mod vector2;

pub fn get_unsigned_int_16s(source: UnsignedInt32) -> (UnsignedInt16, UnsignedInt16) {
    (source as UnsignedInt16, (source >> 16) as UnsignedInt16)
}

pub fn get_unsigned_int_8s(
    source: UnsignedInt32,
) -> (UnsignedInt8, UnsignedInt8, UnsignedInt8, UnsignedInt8) {
    (
        source as UnsignedInt8,
        (source >> 8) as UnsignedInt8,
        (source >> 16) as UnsignedInt8,
        (source >> 24) as UnsignedInt8,
    )
}
