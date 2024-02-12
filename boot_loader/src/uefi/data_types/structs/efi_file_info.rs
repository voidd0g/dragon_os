use crate::uefi::data_types::basic_types::{CHAR16, UINT64};

use super::efi_time::EFI_TIME;

#[repr(C)]
pub struct EFI_FILE_INFO {
    Size: UINT64,
    FileSize: UINT64,
    PhysicalSize: UINT64,
    CreateTime: EFI_TIME,
    LastAccessTime: EFI_TIME,
    ModificationTime: EFI_TIME,
    Attribute: UINT64,
    FileName: *const CHAR16,
}

#[deny(non_snake_case)]
impl EFI_FILE_INFO {
    pub fn file_size(&self) -> UINT64 {
        self.FileSize
    }

    pub fn attribute(&self) -> UINT64 {
        self.Attribute
    }
}
