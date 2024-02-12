#![allow(non_upper_case_globals)]

use crate::uefi::data_types::basic_types::EFI_MEMORY_TYPE;

pub const EfiReservedMemoryType: EFI_MEMORY_TYPE = 0;
pub const EfiLoaderCode: EFI_MEMORY_TYPE = 1;
pub const EfiLoaderData: EFI_MEMORY_TYPE = 2;
pub const EfiBootServicesCode: EFI_MEMORY_TYPE = 3;
pub const EfiBootServicesData: EFI_MEMORY_TYPE = 4;
pub const EfiRuntimeServicesCode: EFI_MEMORY_TYPE = 5;
pub const EfiRuntimeServicesData: EFI_MEMORY_TYPE = 6;
pub const EfiConventionalMemory: EFI_MEMORY_TYPE = 7;
pub const EfiUnusableMemory: EFI_MEMORY_TYPE = 8;
pub const EfiACPIReclaimMemory: EFI_MEMORY_TYPE = 9;
pub const EfiACPIMemoryNVS: EFI_MEMORY_TYPE = 10;
pub const EfiMemoryMappedIO: EFI_MEMORY_TYPE = 11;
pub const EfiMemoryMappedIOPortSpace: EFI_MEMORY_TYPE = 12;
pub const EfiPalCode: EFI_MEMORY_TYPE = 13;
pub const EfiPersistentMemory: EFI_MEMORY_TYPE = 14;
pub const EfiUnacceptedMemoryType: EFI_MEMORY_TYPE = 15;
pub const EfiMaxMemoryType: EFI_MEMORY_TYPE = 16;
