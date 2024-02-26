use crate::uefi::data_type::basic_type::EfiResetType;

pub const EFI_RESET_COLD: EfiResetType = 0;
pub const EFI_RESET_WARM: EfiResetType = 1;
pub const EFI_RESET_SHUTDOWN: EfiResetType = 2;
pub const EFI_RESET_PLATFORM_SPECIFIC: EfiResetType = 3;
