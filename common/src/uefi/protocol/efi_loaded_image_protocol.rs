use crate::uefi::{
    data_type::basic_type::{EfiHandle, EFI_MEMORY_TYPE, EfiStatus, UnsignedInt32, UnsignedInt64, Void},
    table::efi_system_table::EfiSystemTable,
};

use super::efi_device_path_protocol::EFI_DEVICE_PATH_PROTOCOL;

type EfiImageUnload = unsafe extern "efiapi" fn(ImageHandle: EfiHandle) -> EfiStatus;

#[repr(C)]
pub struct EfiLoadedImageProtocol {
    Revision: UnsignedInt32,
    ParentHandle: EfiHandle,
    SystemTable: *const EfiSystemTable,

    DeviceHandle: EfiHandle,
    FilePath: *const EFI_DEVICE_PATH_PROTOCOL,
    Reserved: *const Void,

    LoadOptionsSize: UnsignedInt32,
    LoadOptions: *const Void,

    ImageBase: *const Void,
    ImageSize: UnsignedInt64,
    ImageCodeType: EFI_MEMORY_TYPE,
    ImageDataType: EFI_MEMORY_TYPE,
    Unload: EfiImageUnload,
}

impl EfiLoadedImageProtocol {
    pub fn device_handle(&self) -> EfiHandle {
        self.DeviceHandle
    }
}
