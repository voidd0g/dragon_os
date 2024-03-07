use super::basic_type::{EfiPhysicalAddress, EfiVirtualAddress};

#[repr(C)]
pub struct EfiMemoryDescriptor {
    r#type: u32,
    physical_start: EfiPhysicalAddress,
    virtual_start: EfiVirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

impl EfiMemoryDescriptor {
    pub fn r#type(&self) -> u32 {
        self.r#type
    }
    pub fn physical_start(&self) -> EfiPhysicalAddress {
        self.physical_start
    }
    pub fn virtual_start(&self) -> EfiVirtualAddress {
        self.virtual_start
    }
    pub fn number_of_pages(&self) -> u64 {
        self.number_of_pages
    }
    pub fn attribute(&self) -> u64 {
        self.attribute
    }
}

pub const UEFI_PAGE_FRAME_SIZE: usize = 0x1000;