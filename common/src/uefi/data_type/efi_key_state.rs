use super::basic_type::EfiKeyToggleState;

#[repr(C)]
pub struct EfiKeyState {
    key_shift_state: u32,
    key_toggle_state: EfiKeyToggleState,
}
