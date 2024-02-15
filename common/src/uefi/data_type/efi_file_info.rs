use super::{
    basic_type::{Char16, UnsignedInt64},
    efi_time::EfiTime,
};

#[repr(C)]
pub struct EfiFileInfo {
    size: UnsignedInt64,
    file_size: UnsignedInt64,
    physical_size: UnsignedInt64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
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
