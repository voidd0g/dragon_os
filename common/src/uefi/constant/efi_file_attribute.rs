use crate::uefi::data_type::basic_type::UnsignedInt64;

pub const EFI_FILE_READ_ONLY: UnsignedInt64 = 0x0000000000000001;
pub const EFI_FILE_HIDDEN: UnsignedInt64 = 0x0000000000000002;
pub const EFI_FILE_SYSTEM: UnsignedInt64 = 0x0000000000000004;
pub const EFI_FILE_RESERVED: UnsignedInt64 = 0x0000000000000008;
pub const EFI_FILE_DIRECTORY: UnsignedInt64 = 0x0000000000000010;
pub const EFI_FILE_ARCHIVE: UnsignedInt64 = 0x0000000000000020;
pub const EFI_FILE_VALID_ATTR: UnsignedInt64 = 0x0000000000000037;
