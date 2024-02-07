use crate::uefi::data_types::common_types::{BOOLEAN, CHAR16, EFI_STATUS};

type EFI_TEXT_RESET = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
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