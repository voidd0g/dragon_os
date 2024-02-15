use crate::uefi::data_type::basic_type::EfiMemoryType;

pub const EFI_RESERVED_MEMORY_TYPE: EfiMemoryType = 0;
pub const EFI_LOADER_CODE: EfiMemoryType = 1;
pub const EFI_LOADER_DATA: EfiMemoryType = 2;
pub const EFI_BOOT_SERVICES_CODE: EfiMemoryType = 3;
pub const EFI_BOOT_SERVICES_DATA: EfiMemoryType = 4;
pub const EFI_RUNTIME_SERVICES_CODE: EfiMemoryType = 5;
pub const EFI_RUNTIME_SERVICES_DATA: EfiMemoryType = 6;
pub const EFI_CONVENTIONAL_MEMORY: EfiMemoryType = 7;
pub const EFI_UNUSABLE_MEMORY: EfiMemoryType = 8;
pub const EFI_ACPIRECLAIM_MEMORY: EfiMemoryType = 9;
pub const EFI_ACPIMEMORY_NVS: EfiMemoryType = 10;
pub const EFI_MEMORY_MAPPED_IO: EfiMemoryType = 11;
pub const EFI_MEMORY_MAPPED_IOPORT_SPACE: EfiMemoryType = 12;
pub const EFI_PAL_CODE: EfiMemoryType = 13;
pub const EFI_PERSISTENT_MEMORY: EfiMemoryType = 14;
pub const EFI_UNACCEPTED_MEMORY_TYPE: EfiMemoryType = 15;
pub const EFI_MAX_MEMORY_TYPE: EfiMemoryType = 16;
