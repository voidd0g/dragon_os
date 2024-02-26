#[repr(C)]
pub struct EfiInputKey {
    scan_code: u16,
    unicode_char: u16,
}
