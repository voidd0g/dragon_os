use crate::uefi::data_type::basic_type::{EFI_KEY_TOGGLE_STATE, UnsignedInt32};

#[repr(C)]
pub struct EFI_KEY_STATE {
    KeyShiftState: UnsignedInt32,
    KeyToggleState: EFI_KEY_TOGGLE_STATE,
}
