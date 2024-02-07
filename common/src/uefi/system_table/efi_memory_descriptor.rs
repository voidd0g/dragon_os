use crate::uefi::data_types::{EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UINT32, UINT64};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}