use crate::uefi::data_type::{
    basic_type::{Boolean, EFI_EVENT, EFI_KEY_TOGGLE_STATE, EfiStatus, Void},
    efi_key_data::EFI_KEY_DATA,
};

type EFI_INPUT_RESET_EX = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    ExtendedVerification: Boolean,
) -> EfiStatus;
type EFI_INPUT_READ_KEY_EX = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyDataOut: *mut EFI_KEY_DATA,
) -> EfiStatus;
type EFI_SET_STATE = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyToggleState: *const EFI_KEY_TOGGLE_STATE,
) -> EfiStatus;
type EFI_REGISTER_KEYSTROKE_NOTIFY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyData: *const EFI_KEY_DATA,
    KeyNotificationFunction: EFI_KEY_NOTIFY_FUNCTION,
    NotifyHandleOut: *mut Void,
) -> EfiStatus;
type EFI_KEY_NOTIFY_FUNCTION =
    unsafe extern "efiapi" fn(KeyData: *const EFI_KEY_DATA) -> EfiStatus;
type EFI_UNREGISTER_KEYSTROKE_NOTIFY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    NotifyHandle: *const Void,
) -> EfiStatus;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    Reset: EFI_INPUT_RESET_EX,
    ReadKeyStrokeEx: EFI_INPUT_READ_KEY_EX,
    WaitForKeyEx: EFI_EVENT,
    SetState: EFI_SET_STATE,
    RegisterKeyNotify: EFI_REGISTER_KEYSTROKE_NOTIFY,
    UnregisterKeyNotify: EFI_UNREGISTER_KEYSTROKE_NOTIFY,
}
