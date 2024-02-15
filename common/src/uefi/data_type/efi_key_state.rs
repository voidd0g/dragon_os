use super::basic_type::{EfiKeyToggleState, UnsignedInt32};

#[repr(C)]
pub struct EfiKeyState {
    key_shift_state: UnsignedInt32,
    key_toggle_state: EfiKeyToggleState,
}
