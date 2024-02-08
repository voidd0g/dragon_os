use crate::uefi::data_types::common_types::{EFI_HANDLE, UINT32};

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
    AgentHandle: EFI_HANDLE,
    ControllerHandle: EFI_HANDLE,
    Attributes: UINT32,
    OpenCount: UINT32,
}