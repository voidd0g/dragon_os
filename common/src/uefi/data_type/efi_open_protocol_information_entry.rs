use super::basic_type::{EfiHandle, UnsignedInt32};

#[repr(C)]
pub struct EfiOpenProtocolInformationEntry {
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: UnsignedInt32,
    open_count: UnsignedInt32,
}
