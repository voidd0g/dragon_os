use crate::uefi::data_types::basic_types::{EFI_KEY_TOGGLE_STATE, UINT32};

#[repr(C)]
pub struct EFI_KEY_STATE {
    KeyShiftState: UINT32,
    KeyToggleState: EFI_KEY_TOGGLE_STATE,
}
