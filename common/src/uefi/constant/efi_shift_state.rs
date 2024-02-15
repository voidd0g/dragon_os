use crate::uefi::data_type::basic_type::UnsignedInt32;

pub const EFI_SHIFT_STATE_VALID: UnsignedInt32 = 0x80000000;
pub const EFI_RIGHT_SHIFT_PRESSED: UnsignedInt32 = 0x00000001;
pub const EFI_LEFT_SHIFT_PRESSED: UnsignedInt32 = 0x00000002;
pub const EFI_RIGHT_CONTROL_PRESSED: UnsignedInt32 = 0x00000004;
pub const EFI_LEFT_CONTROL_PRESSED: UnsignedInt32 = 0x00000008;
pub const EFI_RIGHT_ALT_PRESSED: UnsignedInt32 = 0x00000010;
pub const EFI_LEFT_ALT_PRESSED: UnsignedInt32 = 0x00000020;
pub const EFI_RIGHT_LOGO_PRESSED: UnsignedInt32 = 0x00000040;
pub const EFI_LEFT_LOGO_PRESSED: UnsignedInt32 = 0x00000080;
pub const EFI_MENU_KEY_PRESSED: UnsignedInt32 = 0x00000100;
pub const EFI_SYS_REQ_PRESSED: UnsignedInt32 = 0x00000200;
