use crate::uefi::data_types::basic_types::{CHAR16, UINT16};

#[repr(C)]
pub struct EFI_INPUT_KEY {
    ScanCode: UINT16,
    UnicodeChar: CHAR16,
}
