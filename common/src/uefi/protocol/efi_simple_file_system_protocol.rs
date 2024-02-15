use core::ptr::null;

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::basic_type::{EfiStatus, UnsignedInt64},
};

use super::efi_file_protocol::EFI_FILE_PROTOCOL;

type EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME = unsafe extern "efiapi" fn(
    This: *const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
    RootOut: *mut *const EFI_FILE_PROTOCOL,
) -> EfiStatus;

#[repr(C)]
pub struct EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    Revision: UnsignedInt64,
    OpenVolume: EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_OPEN_VOLUME,
}

impl EFI_SIMPLE_FILE_SYSTEM_PROTOCOL {
    pub fn open_volume(&self) -> Result<&EFI_FILE_PROTOCOL, EfiStatus> {
        let mut root_out = null();
        let status = unsafe { (self.OpenVolume)(self, &mut root_out) };
        match status {
            EFI_SUCCESS => Ok(unsafe { root_out.as_ref() }.unwrap()),
            v => Err(v),
        }
    }
}
