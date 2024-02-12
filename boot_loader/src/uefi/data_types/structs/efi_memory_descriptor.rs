use crate::uefi::data_types::basic_types::{EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UINT32, UINT64};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}

#[deny(non_snake_case)]
impl EFI_MEMORY_DESCRIPTOR {
	pub fn r#type(&self) -> UINT32 {
		self.Type
	}
	pub fn physical_start(&self) -> EFI_PHYSICAL_ADDRESS {
		self.PhysicalStart
	}
	pub fn virtual_start(&self) -> EFI_VIRTUAL_ADDRESS {
		self.VirtualStart
	}
	pub fn number_of_pages(&self) -> UINT64 {
		self.NumberOfPages
	}
	pub fn attribute(&self) -> UINT64 {
		self.Attribute
	}
}