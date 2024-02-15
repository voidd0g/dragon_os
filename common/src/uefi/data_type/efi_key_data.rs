use super::{efi_input_key::EFI_INPUT_KEY, efi_key_state::EFI_KEY_STATE};

#[repr(C)]
pub struct EFI_KEY_DATA {
    Key: EFI_INPUT_KEY,
    KeyState: EFI_KEY_STATE,
}
