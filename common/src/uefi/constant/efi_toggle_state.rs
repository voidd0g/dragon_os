use crate::uefi::data_type::basic_type::UnsignedInt8;

pub const EFI_TOGGLE_STATE_VALID: UnsignedInt8 = 0x80;
pub const EFI_KEY_STATE_EXPOSED: UnsignedInt8 = 0x40;
pub const EFI_SCROLL_LOCK_ACTIVE: UnsignedInt8 = 0x01;
pub const EFI_NUM_LOCK_ACTIVE: UnsignedInt8 = 0x02;
pub const EFI_CAPS_LOCK_ACTIVE: UnsignedInt8 = 0x04;
