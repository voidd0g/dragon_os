use crate::uefi::data_type::basic_type::UnsignedInt8;

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    Type: UnsignedInt8,
    SubType: UnsignedInt8,
    Length: [UnsignedInt8; 2],
}
