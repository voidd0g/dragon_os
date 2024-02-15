use crate::uefi::{
    data_type::basic_type::{
        EfiHandle, EfiMemoryType, EfiStatus, UnsignedInt32, UnsignedInt64, Void,
    },
    table::efi_system_table::EfiSystemTable,
};

use super::efi_device_path_protocol::EfiDevicePathProtocol;

type EfiImageUnload = unsafe extern "efiapi" fn(image_handle: EfiHandle) -> EfiStatus;

#[repr(C)]
pub struct EfiLoadedImageProtocol {
    revision: UnsignedInt32,
    parent_handle: EfiHandle,
    system_table: *const EfiSystemTable,

    device_handle: EfiHandle,
    file_path: *const EfiDevicePathProtocol,
    reserved: *const Void,

    load_options_size: UnsignedInt32,
    load_options: *const Void,

    image_base: *const Void,
    image_size: UnsignedInt64,
    image_code_type: EfiMemoryType,
    image_data_type: EfiMemoryType,
    unload: EfiImageUnload,
}

impl EfiLoadedImageProtocol {
    pub fn device_handle(&self) -> EfiHandle {
        self.device_handle
    }
}
