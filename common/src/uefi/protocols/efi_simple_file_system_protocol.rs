use core::ptr::null;

use crate::{uefi::data_types::basic_types::{EFI_STATUS, UINT64, UINT8}, utils::from_byte_slice::FromByteSlice};

use super::efi_file_protocol::EFI_FILE_PROTOCOL;

type EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME = unsafe extern "efiapi" fn (This: *const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, RootOut: *mut *const EFI_FILE_PROTOCOL) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL { 
    Revision: UINT64,
    OpenVolume: EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME,
}

#[deny(non_snake_case)]
impl EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    pub fn open_volume(&self) -> (EFI_STATUS, &EFI_FILE_PROTOCOL) {
        let mut root_out = null();
        let status = unsafe {
            (self.OpenVolume)(self, &mut root_out)
        };
        (status, unsafe {
            root_out.as_ref()
        }.unwrap())
    }
}

#[deny(non_snake_case)]
impl FromByteSlice for EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
        let (revision, bs) = UINT64::from_byte_slice(bs);
        let (open_volume, bs) = <*const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME>::from_byte_slice(bs);

        (Self { 
            Revision: revision, 
            OpenVolume: unsafe { open_volume.read() } 
        }, bs)
    }
}