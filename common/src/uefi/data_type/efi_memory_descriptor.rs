use crate::uefi::data_type::basic_type::{
    EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UnsignedInt32, UnsignedInt64,
};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
    Type: UnsignedInt32,
    PhysicalStart: EFI_PHYSICAL_ADDRESS,
    VirtualStart: EFI_VIRTUAL_ADDRESS,
    NumberOfPages: UnsignedInt64,
    Attribute: UnsignedInt64,
}

impl EFI_MEMORY_DESCRIPTOR {
    pub fn r#type(&self) -> UnsignedInt32 {
        self.Type
    }
    pub fn physical_start(&self) -> EFI_PHYSICAL_ADDRESS {
        self.PhysicalStart
    }
    pub fn virtual_start(&self) -> EFI_VIRTUAL_ADDRESS {
        self.VirtualStart
    }
    pub fn number_of_pages(&self) -> UnsignedInt64 {
        self.NumberOfPages
    }
    pub fn attribute(&self) -> UnsignedInt64 {
        self.Attribute
    }
}
