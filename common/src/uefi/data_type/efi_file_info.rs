use crate::uefi::data_type::basic_type::{Char16, UnsignedInt64};

use super::efi_time::EFI_TIME;

#[repr(C)]
pub struct EfiFileInfo {
    size: UnsignedInt64,
    file_size: UnsignedInt64,
    physical_size: UnsignedInt64,
    create_time: EFI_TIME,
    last_access_time: EFI_TIME,
    modification_time: EFI_TIME,
    attribute: UnsignedInt64,
    file_name: *const Char16,
}

impl EfiFileInfo {
    pub fn file_size(&self) -> UnsignedInt64 {
        self.file_size
    }

    pub fn attribute(&self) -> UnsignedInt64 {
        self.attribute
    }
}
