use super::efi_time::EfiTime;

#[repr(C)]
pub struct EfiFileInfo {
    size: u64,
    file_size: u64,
    physical_size: u64,
    create_time: EfiTime,
    last_access_time: EfiTime,
    modification_time: EfiTime,
    attribute: u64,
    file_name: *const u16,
}

impl EfiFileInfo {
    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn attribute(&self) -> u64 {
        self.attribute
    }
}
