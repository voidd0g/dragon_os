use super::{efi_input_key::EfiInputKey, efi_key_state::EfiKeyState};

#[repr(C)]
pub struct EfiKeyData {
    key: EfiInputKey,
    key_state: EfiKeyState,
}
