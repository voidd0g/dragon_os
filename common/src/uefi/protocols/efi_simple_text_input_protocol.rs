use crate::uefi::data_types::{
    basic_types::{BOOLEAN, EFI_EVENT, EFI_STATUS},
    structs::efi_input_key::EFI_INPUT_KEY,
};

type EFI_INPUT_RESET = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;
type EFI_INPUT_READ_KEY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    KeyOut: *mut EFI_INPUT_KEY,
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    Reset: EFI_INPUT_RESET,
    ReadKeyStroke: EFI_INPUT_READ_KEY,
    WaitForKey: EFI_EVENT,
}
