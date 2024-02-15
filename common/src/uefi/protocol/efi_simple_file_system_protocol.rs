use core::ptr::null;

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::basic_type::{EfiStatus, UnsignedInt64},
};

use super::efi_file_protocol::EfiFileProtocol;

type EfiSimpleFileSystemProtocolOpenVolume = unsafe extern "efiapi" fn(
    this: *const EfiSimpleFileSystemProtocol,
    root_out: *mut *const EfiFileProtocol,
) -> EfiStatus;

#[repr(C)]
pub struct EfiSimpleFileSystemProtocol {
    revision: UnsignedInt64,
    open_volume: EfiSimpleFileSystemProtocolOpenVolume,
}

impl EfiSimpleFileSystemProtocol {
    pub fn open_volume(&self) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut root_out = null();
        let status = unsafe { (self.open_volume)(self, &mut root_out) };
        match status {
            EFI_SUCCESS => Ok(unsafe { root_out.as_ref() }.unwrap()),
            v => Err(v),
        }
    }
}
