use crate::{uefi::{data_types::basic_types::{EFI_HANDLE, EFI_MEMORY_TYPE, EFI_STATUS, UINT32, UINT64, UINT8, VOID}, tables::efi_system_table::EFI_SYSTEM_TABLE}, utils::from_byte_slice::FromByteSlice};

use super::efi_device_path_protocol::EFI_DEVICE_PATH_PROTOCOL;

type EFI_IMAGE_UNLOAD = extern "C" fn (ImageHandle: EFI_HANDLE) -> EFI_STATUS;

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

#[deny(non_snake_case)]
impl FromByteSlice for EFI_LOADED_IMAGE_PROTOCOL {
    fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
        let (revision, bs) = UINT32::from_byte_slice(bs);
        let (parent_handle, bs) = EFI_HANDLE::from_byte_slice(bs);
        let (system_table, bs) = <*const EFI_SYSTEM_TABLE>::from_byte_slice(bs);

        let (device_handle, bs) = EFI_HANDLE::from_byte_slice(bs);
        let (file_path, bs) = <*const EFI_DEVICE_PATH_PROTOCOL>::from_byte_slice(bs);
        let (reserved, bs) = <*const VOID>::from_byte_slice(bs);
    
        let (load_options_size, bs) = UINT32::from_byte_slice(bs);
        let (load_options, bs) = <*const VOID>::from_byte_slice(bs);
    
        let (image_base, bs) = <*const VOID>::from_byte_slice(bs);
        let (image_size, bs) = UINT64::from_byte_slice(bs);
        let (image_code_type, bs) = EFI_MEMORY_TYPE::from_byte_slice(bs);
        let (image_data_type, bs) = EFI_MEMORY_TYPE::from_byte_slice(bs);
        let (unload, bs) = <*const EFI_IMAGE_UNLOAD>::from_byte_slice(bs);

        (Self { 
            Revision: revision, 
            ParentHandle: parent_handle, 
            SystemTable: system_table, 
            DeviceHandle: device_handle, 
            FilePath: file_path, 
            Reserved: reserved, 
            LoadOptionsSize: load_options_size, 
            LoadOptions: load_options, 
            ImageBase: image_base, 
            ImageSize: image_size, 
            ImageCodeType: image_code_type, 
            ImageDataType: image_data_type, 
            Unload: unsafe { unload.read() } 
        }, bs)
    }
}