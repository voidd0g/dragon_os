use crate::uefi::data_type::basic_type::UnsignedInt64;

pub const EFI_FILE_PROTOCOL_REVISION: UnsignedInt64 = 0x00010000;
pub const EFI_FILE_PROTOCOL_REVISION2: UnsignedInt64 = 0x00020000;
pub const EFI_FILE_PROTOCOL_LATEST_REVISION: UnsignedInt64 = EFI_FILE_PROTOCOL_REVISION2;
