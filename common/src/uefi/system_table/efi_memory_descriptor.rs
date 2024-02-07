use crate::uefi::data_types::{common_types::{UINT32, UINT64}, other_types::{EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS}};

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/07_Services_Boot_Services.html#efi-boot-services-getmemorymap
/// "Related Definitions"
#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}