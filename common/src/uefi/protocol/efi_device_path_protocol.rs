use crate::uefi::data_type::basic_type::UnsignedInt8;

#[repr(C)]
pub struct EfiDevicePathProtocol {
    r#type: UnsignedInt8,
    sub_type: UnsignedInt8,
    length: [UnsignedInt8; 2],
}
