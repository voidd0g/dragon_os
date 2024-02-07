use crate::uefi::data_types::common_types::{BOOLEAN, CHAR16, EFI_GUID, EFI_STATUS, UINT64};

type EFI_TEXT_RESET = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    _Unuse0: UINT64,
    _Unuse1: UINT64,
    _Unuse2: UINT64,
    _Unuse3: UINT64,
    _Unuse4: UINT64,
    _Unuse5: UINT64,
    _Unuse6: UINT64,
    _Unuse7: UINT64,
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

pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EFI_GUID = EFI_GUID::new(0x387477c2, 0x69c7, 0x11d2, [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b]);