use crate::uefi::data_type::{
    basic_type::{Boolean, EfiEvent, EfiStatus},
    efi_input_key::EfiInputKey,
};

type EfiInputReset = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputProtocol,
    extended_verification: Boolean,
) -> EfiStatus;
type EfiInputReadKey = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputProtocol,
    key_out: *mut EfiInputKey,
) -> EfiStatus;

#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    reset: EfiInputReset,
    read_key_stroke: EfiInputReadKey,
    wait_for_key: EfiEvent,
}
