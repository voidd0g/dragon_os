pub mod vector2;

pub fn get_unsigned_int_16s(source: u32) -> (u16, u16) {
    (source as u16, (source >> 16) as u16)
}

pub fn get_unsigned_int_8s(source: u32) -> (u8, u8, u8, u8) {
    (
        source as u8,
        (source >> 8) as u8,
        (source >> 16) as u8,
        (source >> 24) as u8,
    )
}
