use crate::uefi::{data_types::basic_types::{EFI_HANDLE, EFI_MEMORY_TYPE, EFI_STATUS, UINT32, UINT64, VOID}, tables::efi_system_table::EFI_SYSTEM_TABLE};

use super::efi_device_path_protocol::EFI_DEVICE_PATH_PROTOCOL;

type EFI_IMAGE_UNLOAD = unsafe extern "efiapi" fn (ImageHandle: EFI_HANDLE) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_LOADED_IMAGE_PROTOCOL { 
    Revision: UINT32,
    ParentHandle: EFI_HANDLE,
    SystemTable: *const EFI_SYSTEM_TABLE,

    DeviceHandle: EFI_HANDLE,
    FilePath: *const EFI_DEVICE_PATH_PROTOCOL,
    Reserved: *const VOID,

    LoadOptionsSize: UINT32,
    LoadOptions: *const VOID,

    ImageBase: *const VOID,
    ImageSize: UINT64,
    ImageCodeType: EFI_MEMORY_TYPE,
    ImageDataType: EFI_MEMORY_TYPE,
    Unload: EFI_IMAGE_UNLOAD,
}

#[deny(non_snake_case)]
impl EFI_LOADED_IMAGE_PROTOCOL {
    pub fn device_handle(&self) -> EFI_HANDLE {
        self.DeviceHandle
    }
}