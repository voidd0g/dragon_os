#![allow(non_upper_case_globals)]

use crate::uefi::data_types::basic_types::EFI_LOCATE_SEARCH_TYPE;

pub const AllHandles: EFI_LOCATE_SEARCH_TYPE = 0;
pub const ByRegisterNotify: EFI_LOCATE_SEARCH_TYPE = 1;
pub const ByProtocol: EFI_LOCATE_SEARCH_TYPE = 2;