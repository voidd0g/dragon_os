use crate::uefi::data_type::basic_type::UnsignedInt64;

pub const EFI_FILE_MODE_READ: UnsignedInt64 = 0x0000000000000001;
pub const EFI_FILE_MODE_WRITE: UnsignedInt64 = 0x0000000000000002;
pub const EFI_FILE_MODE_CREATE: UnsignedInt64 = 0x8000000000000000;
