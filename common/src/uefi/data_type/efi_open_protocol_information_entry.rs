use super::basic_type::EfiHandle;

#[repr(C)]
pub struct EfiOpenProtocolInformationEntry {
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: u32,
    open_count: u32,
}
