#![allow(non_upper_case_globals)]

use crate::uefi::data_type::basic_type::EfiAllocateType;

pub const AllocateAnyPages: EfiAllocateType = 0;
pub const AllocateMaxAddress: EfiAllocateType = 1;
pub const AllocateAddress: EfiAllocateType = 2;
pub const MaxAllocateType: EfiAllocateType = 3;
