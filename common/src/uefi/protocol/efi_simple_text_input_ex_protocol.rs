use crate::uefi::data_type::{
    basic_type::{Boolean, EfiEvent, EfiKeyToggleState, EfiStatus, Void},
    efi_key_data::EfiKeyData,
};

type EfiInputResetEx = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputExProtocol,
    extended_verification: Boolean,
) -> EfiStatus;
type EfiInputReadKeyEx = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputExProtocol,
    key_data_out: *mut EfiKeyData,
) -> EfiStatus;
type EfiSetState = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputExProtocol,
    key_toggle_state: *const EfiKeyToggleState,
) -> EfiStatus;
type EfiRegisterKeystrokeNotify = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputExProtocol,
    key_data: *const EfiKeyData,
    key_notification_function: EfiKeyNotifyFunction,
    notify_handle_out: *mut Void,
) -> EfiStatus;
type EfiKeyNotifyFunction = unsafe extern "efiapi" fn(key_data: *const EfiKeyData) -> EfiStatus;
type EfiUnregisterKeystrokeNotify = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextInputExProtocol,
    notify_handle: *const Void,
) -> EfiStatus;

#[repr(C)]
pub struct EfiSimpleTextInputExProtocol {
    reset: EfiInputResetEx,
    read_key_stroke_ex: EfiInputReadKeyEx,
    wait_for_key_ex: EfiEvent,
    set_state: EfiSetState,
    register_key_notify: EfiRegisterKeystrokeNotify,
    unregister_key_notify: EfiUnregisterKeystrokeNotify,
}
