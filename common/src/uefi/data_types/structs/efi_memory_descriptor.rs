use crate::{uefi::data_types::common_types::{EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UINT32, UINT64, UINT8}, utils::from_byte_slice::FromByteSlice};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}

impl FromByteSlice for EFI_MEMORY_DESCRIPTOR {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) {
		
	}
}