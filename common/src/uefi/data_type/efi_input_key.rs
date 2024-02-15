use crate::uefi::data_type::basic_type::{Char16, UnsignedInt16};

#[repr(C)]
pub struct EFI_INPUT_KEY {
    ScanCode: UnsignedInt16,
    UnicodeChar: Char16,
}
