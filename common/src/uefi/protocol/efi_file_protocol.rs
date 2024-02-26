use core::ptr::null;

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::{
        basic_type::{
            Char16, EfiGuid, EfiStatus, UnsignedInt64, UnsignedInt8, UnsignedIntNative, Void,
        },
        efi_file_io_token::EfiFileIoToken,
    },
};

type EfiFileOpen = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    new_handle_out: *mut *const EfiFileProtocol,
    file_name: *const Char16,
    open_mode: UnsignedInt64,
    attributes: UnsignedInt64,
) -> EfiStatus;
type EfiFileClose = unsafe extern "efiapi" fn(This: *const EfiFileProtocol) -> EfiStatus;
type EfiFileDelete = unsafe extern "efiapi" fn(This: *const EfiFileProtocol) -> EfiStatus;
type EfiFileRead = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    buffer_size_in_out: *mut UnsignedIntNative,
    buffer_out: *mut Void,
) -> EfiStatus;
type EfiFileWrite = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    buffer_size_in_out: *mut UnsignedIntNative,
    buffer: *const Void,
) -> EfiStatus;
type EfiFileGetPosition = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    position_out: *mut UnsignedInt64,
) -> EfiStatus;
type EfiFileSetPosition =
    unsafe extern "efiapi" fn(This: *const EfiFileProtocol, Position: UnsignedInt64) -> EfiStatus;
type EfiFileGetInfo = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    information_type: *const EfiGuid,
    buffer_size_in_out: *mut UnsignedIntNative,
    buffer_out: *mut Void,
) -> EfiStatus;
type EfiFileSetInfo = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    information_type: *const EfiGuid,
    buffer_size: UnsignedIntNative,
    buffer: *const Void,
) -> EfiStatus;
type EfiFileFlush = unsafe extern "efiapi" fn(This: *const EfiFileProtocol) -> EfiStatus;
type EfiFileOpenEx = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    new_handle_out: *mut *const EfiFileProtocol,
    file_name: *const Char16,
    open_mode: UnsignedInt64,
    attributes: UnsignedInt64,
    token_in_out: *mut EfiFileIoToken,
) -> EfiStatus;
type EfiFileReadEx = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    token_in_out: *mut EfiFileIoToken,
) -> EfiStatus;
type EfiFileWriteEx = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    token_in_out: *mut EfiFileIoToken,
) -> EfiStatus;
type EfiFileFlushEx = unsafe extern "efiapi" fn(
    this: *const EfiFileProtocol,
    token_in_out: *mut EfiFileIoToken,
) -> EfiStatus;

#[repr(C)]
pub struct EfiFileProtocol {
    revision: UnsignedInt64,
    open: EfiFileOpen,
    close: EfiFileClose,
    delete: EfiFileDelete,
    read: EfiFileRead,
    write: EfiFileWrite,
    get_position: EfiFileGetPosition,
    set_position: EfiFileSetPosition,
    get_info: EfiFileGetInfo,
    set_info: EfiFileSetInfo,
    flush: EfiFileFlush,
    open_ex: EfiFileOpenEx,
    read_ex: EfiFileReadEx,
    write_ex: EfiFileWriteEx,
    flush_ex: EfiFileFlushEx,
}

impl EfiFileProtocol {
    pub fn open(
        &self,
        file_name: &[Char16],
        open_mode: UnsignedInt64,
        attributes: UnsignedInt64,
    ) -> Result<&EfiFileProtocol, EfiStatus> {
        let mut new_handle_out = null();
        let status = unsafe {
            (self.open)(
                self,
                &mut new_handle_out,
                file_name.as_ptr(),
                open_mode,
                attributes,
            )
        };
        match status {
            EFI_SUCCESS => Ok(unsafe { new_handle_out.as_ref() }.unwrap()),
            v => Err(v),
        }
    }
    pub fn close(&self) -> Result<(), EfiStatus> {
        let status = unsafe { (self.close)(self) };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }

    pub fn read(
        &self,
        buffer_size_in_out: &mut UnsignedIntNative,
        buffer_out: &mut [UnsignedInt8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.read)(
                self,
                buffer_size_in_out,
                buffer_out.as_mut_ptr() as *mut Void,
            )
        };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }
    pub fn write(
        &self,
        buffer_size_in_out: &mut UnsignedIntNative,
        buffer: &[UnsignedInt8],
    ) -> Result<(), EfiStatus> {
        let status =
            unsafe { (self.write)(self, buffer_size_in_out, buffer.as_ptr() as *const Void) };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }

    pub fn get_info(
        &self,
        information_type: &EfiGuid,
        buffer_size_in_out: &mut UnsignedIntNative,
        buffer_out: &mut [UnsignedInt8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.get_info)(
                self,
                information_type,
                buffer_size_in_out,
                buffer_out.as_ptr() as *mut Void,
            )
        };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }
    pub fn set_info(
        &self,
        information_type: &EfiGuid,
        buffer_size: UnsignedIntNative,
        buffer: &[UnsignedInt8],
    ) -> Result<(), EfiStatus> {
        let status = unsafe {
            (self.set_info)(
                self,
                information_type,
                buffer_size,
                buffer.as_ptr() as *const Void,
            )
        };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }
}
