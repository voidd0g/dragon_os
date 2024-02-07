use super::common_types::{EFI_HANDLE, EFI_PHYSICAL_ADDRESS, EFI_VIRTUAL_ADDRESS, UINT16, UINT32, UINT64, UINT8};

#[repr(C)]
pub struct EFI_MEMORY_DESCRIPTOR {
	Type: UINT32,
	PhysicalStart: EFI_PHYSICAL_ADDRESS,
	VirtualStart: EFI_VIRTUAL_ADDRESS,
	NumberOfPages: UINT64,
	Attribute: UINT64,
}

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
    AgentHandle: EFI_HANDLE,
    ControllerHandle: EFI_HANDLE,
    Attributes: UINT32,
    OpenCount: UINT32,
}