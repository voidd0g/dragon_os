#[repr(C)]
pub struct EfiDevicePathProtocol {
    r#type: u8,
    sub_type: u8,
    length: [u8; 2],
}
