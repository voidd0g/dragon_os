use super::basic_type::{EfiPhysicalAddress, EfiVirtualAddress, UnsignedInt32, UnsignedInt64};

#[repr(C)]
pub struct EfiMemoryDescriptor {
    r#type: UnsignedInt32,
    physical_start: EfiPhysicalAddress,
    virtual_start: EfiVirtualAddress,
    number_of_pages: UnsignedInt64,
    attribute: UnsignedInt64,
}

impl EfiMemoryDescriptor {
    pub fn r#type(&self) -> UnsignedInt32 {
        self.r#type
    }
    pub fn physical_start(&self) -> EfiPhysicalAddress {
        self.physical_start
    }
    pub fn virtual_start(&self) -> EfiVirtualAddress {
        self.virtual_start
    }
    pub fn number_of_pages(&self) -> UnsignedInt64 {
        self.number_of_pages
    }
    pub fn attribute(&self) -> UnsignedInt64 {
        self.attribute
    }
}
