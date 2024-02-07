use crate::uefi::{data_types::{CHAR16, EFI_STATUS}, function_types::{EFI_TEXT_RESET, EFI_TEXT_STRING}};

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    _Unuse0: u64,
    _Unuse1: u64,
    _Unuse2: u64,
    _Unuse3: u64,
    _Unuse4: u64,
    _Unuse5: u64,
    _Unuse6: u64,
    _Unuse7: u64,
}

#[deny(non_snake_case)]
impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub fn reset(&self, extended_verification: bool) -> EFI_STATUS {
        (self.Reset)(self, if extended_verification { 1u8 } else { 0u8 })
    }

    pub fn output_string(&self, string: &[CHAR16]) -> EFI_STATUS {
        (self.OutputString)(self, string.as_ptr())
    }
}