use crate::uefi::data_types::{
    basic_types::{BOOLEAN, EFI_EVENT, EFI_KEY_TOGGLE_STATE, EFI_STATUS, VOID},
    structs::efi_key_data::EFI_KEY_DATA,
};

type EFI_INPUT_RESET_EX = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    ExtendedVerification: BOOLEAN,
) -> EFI_STATUS;
type EFI_INPUT_READ_KEY_EX = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyDataOut: *mut EFI_KEY_DATA,
) -> EFI_STATUS;
type EFI_SET_STATE = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyToggleState: *const EFI_KEY_TOGGLE_STATE,
) -> EFI_STATUS;
type EFI_REGISTER_KEYSTROKE_NOTIFY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    KeyData: *const EFI_KEY_DATA,
    KeyNotificationFunction: EFI_KEY_NOTIFY_FUNCTION,
    NotifyHandleOut: *mut VOID,
) -> EFI_STATUS;
type EFI_KEY_NOTIFY_FUNCTION =
    unsafe extern "efiapi" fn(KeyData: *const EFI_KEY_DATA) -> EFI_STATUS;
type EFI_UNREGISTER_KEYSTROKE_NOTIFY = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL,
    NotifyHandle: *const VOID,
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL {
    Reset: EFI_INPUT_RESET_EX,
    ReadKeyStrokeEx: EFI_INPUT_READ_KEY_EX,
    WaitForKeyEx: EFI_EVENT,
    SetState: EFI_SET_STATE,
    RegisterKeyNotify: EFI_REGISTER_KEYSTROKE_NOTIFY,
    UnregisterKeyNotify: EFI_UNREGISTER_KEYSTROKE_NOTIFY,
}
