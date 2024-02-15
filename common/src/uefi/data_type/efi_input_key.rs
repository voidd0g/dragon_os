use super::basic_type::{Char16, UnsignedInt16};

#[repr(C)]
pub struct EfiInputKey {
    scan_code: UnsignedInt16,
    unicode_char: Char16,
}
