use crate::uefi::data_types::basic_types::UINT8;

#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    Type: UINT8,
    SubType: UINT8,
    Length: [UINT8; 2],
}
