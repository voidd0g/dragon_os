use crate::uefi::data_type::{
    basic_type::{Boolean, EFI_EVENT, EfiStatus},
    efi_input_key::EFI_INPUT_KEY,
};

type EFI_INPUT_RESET = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ExtendedVerification: Boolean,
) -> EfiStatus;
type EFI_INPUT_READ_KEY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    KeyOut: *mut EFI_INPUT_KEY,
) -> EfiStatus;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_PROTOCOL {
    Reset: EFI_INPUT_RESET,
    ReadKeyStroke: EFI_INPUT_READ_KEY,
    WaitForKey: EFI_EVENT,
}
