#![allow(non_upper_case_globals)]

use crate::uefi::data_types::basic_types::EFI_ALLOCATE_TYPE;

pub const AllocateAnyPages: EFI_ALLOCATE_TYPE = 0;
pub const AllocateMaxAddress: EFI_ALLOCATE_TYPE = 1;
pub const AllocateAddress: EFI_ALLOCATE_TYPE = 2;
pub const MaxAllocateType: EFI_ALLOCATE_TYPE = 3;