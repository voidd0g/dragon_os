use crate::{uefi::data_types::basic_types::{EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UINT32, UINT64, UINT8}, utils::from_byte_slice::FromByteSlice};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}

#[deny(non_snake_case)]
impl FromByteSlice for EFI_MEMORY_DESCRIPTOR {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let (r#type, bs) = UINT32::from_byte_slice(bs);
		let (physical_start, bs) = UINT64::from_byte_slice(bs);
		let (virtual_start, bs) = UINT64::from_byte_slice(bs);
		let (number_of_pages, bs) = UINT64::from_byte_slice(bs);
		let (attribute, bs) = UINT64::from_byte_slice(bs);
		(Self { Type: r#type, PhysicalStart: physical_start, VirtualStart: virtual_start, NumberOfPages: number_of_pages, Attribute: attribute }, bs)
	}
}