use crate::uefi::data_types::basic_types::UINT8;

/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/10_Protocols_Device_Path_Protocol.html#efi-device-path-protocol
#[repr(C)]
pub struct EFI_DEVICE_PATH_PROTOCOL {
    Type: UINT8,
    SubType: UINT8,
    Length: [UINT8; 2],
}
