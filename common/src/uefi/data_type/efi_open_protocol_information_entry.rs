use crate::uefi::data_type::basic_type::{EfiHandle, UnsignedInt32};

#[repr(C)]
pub struct EFI_OPEN_PROTOCOL_INFORMATION_ENTRY {
    AgentHandle: EfiHandle,
    ControllerHandle: EfiHandle,
    Attributes: UnsignedInt32,
    OpenCount: UnsignedInt32,
}
