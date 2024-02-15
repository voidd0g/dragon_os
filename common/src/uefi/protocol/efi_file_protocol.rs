use core::ptr::null;

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::{
        basic_type::{Char16, EFI_GUID, EfiStatus, UnsignedInt64, UnsignedInt8, UnsignedIntNative, Void},
        efi_file_io_token::EFI_FILE_IO_TOKEN,
    },
};

type EFI_FILE_OPEN = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    NewHandleOut: *mut *const EFI_FILE_PROTOCOL,
    FileName: *const Char16,
    OpenMode: UnsignedInt64,
    Attributes: UnsignedInt64,
) -> EfiStatus;
type EFI_FILE_CLOSE = unsafe extern "efiapi" fn(This: *const EFI_FILE_PROTOCOL) -> EfiStatus;
type EFI_FILE_DELETE = unsafe extern "efiapi" fn(This: *const EFI_FILE_PROTOCOL) -> EfiStatus;
type EFI_FILE_READ = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    BufferSizeInOut: *mut UnsignedIntNative,
    BufferOut: *mut Void,
) -> EfiStatus;
type EFI_FILE_WRITE = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    BufferSizeInOut: *mut UnsignedIntNative,
    Buffer: *const Void,
) -> EfiStatus;
type EFI_FILE_GET_POSITION = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    PositionOut: *mut UnsignedInt64,
) -> EfiStatus;
type EFI_FILE_SET_POSITION =
    unsafe extern "efiapi" fn(This: *const EFI_FILE_PROTOCOL, Position: UnsignedInt64) -> EfiStatus;
type EFI_FILE_GET_INFO = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    InformationType: *const EFI_GUID,
    BufferSizeInOut: *mut UnsignedIntNative,
    BufferOut: *mut Void,
) -> EfiStatus;
type EFI_FILE_SET_INFO = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    InformationType: *const EFI_GUID,
    BufferSize: UnsignedIntNative,
    Buffer: *const Void,
) -> EfiStatus;
type EFI_FILE_FLUSH = unsafe extern "efiapi" fn(This: *const EFI_FILE_PROTOCOL) -> EfiStatus;
type EFI_FILE_OPEN_EX = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    NewHandleOut: *mut *const EFI_FILE_PROTOCOL,
    FileName: *const Char16,
    OpenMode: UnsignedInt64,
    Attributes: UnsignedInt64,
    TokenInOut: *mut EFI_FILE_IO_TOKEN,
) -> EfiStatus;
type EFI_FILE_READ_EX = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    TokenInOut: *mut EFI_FILE_IO_TOKEN,
) -> EfiStatus;
type EFI_FILE_WRITE_EX = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    TokenInOut: *mut EFI_FILE_IO_TOKEN,
) -> EfiStatus;
type EFI_FILE_FLUSH_EX = unsafe extern "efiapi" fn(
    This: *const EFI_FILE_PROTOCOL,
    TokenInOut: *mut EFI_FILE_IO_TOKEN,
) -> EfiStatus;

#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
    Revision: UnsignedInt64,
    Open: EFI_FILE_OPEN,
    Close: EFI_FILE_CLOSE,
    Delete: EFI_FILE_DELETE,
    Read: EFI_FILE_READ,
    Write: EFI_FILE_WRITE,
    GetPosition: EFI_FILE_GET_POSITION,
    SetPosition: EFI_FILE_SET_POSITION,
    GetInfo: EFI_FILE_GET_INFO,
    SetInfo: EFI_FILE_SET_INFO,
    Flush: EFI_FILE_FLUSH,
    OpenEx: EFI_FILE_OPEN_EX,
    ReadEx: EFI_FILE_READ_EX,
    WriteEx: EFI_FILE_WRITE_EX,
    FlushEx: EFI_FILE_FLUSH_EX,
}

impl EFI_FILE_PROTOCOL {
    pub fn open(
        &self,
        file_name: &[Char16],
        open_mode: UnsignedInt64,
        attributes: UnsignedInt64,
    ) -> Result<&EFI_FILE_PROTOCOL, EfiStatus> {
        let mut new_handle_out = null();
        let status = unsafe {
            (self.Open)(
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
    pub fn close(&self) -> EfiStatus {
        let status = unsafe { (self.Close)(self) };
        status
    }

    pub fn read(&self, buffer_size_in_out: &mut UnsignedIntNative, buffer_out: &mut [UnsignedInt8]) -> EfiStatus {
        let status = unsafe {
            (self.Read)(
                self,
                buffer_size_in_out,
                buffer_out.as_mut_ptr() as *mut Void,
            )
        };
        status
    }
    pub fn write(&self, buffer_size_in_out: &mut UnsignedIntNative, buffer: &[UnsignedInt8]) -> EfiStatus {
        let status =
            unsafe { (self.Write)(self, buffer_size_in_out, buffer.as_ptr() as *const Void) };
        status
    }

    pub fn get_info(
        &self,
        information_type: &EFI_GUID,
        buffer_size_in_out: &mut UnsignedIntNative,
        buffer_out: &mut [UnsignedInt8],
    ) -> EfiStatus {
        let status = unsafe {
            (self.GetInfo)(
                self,
                information_type,
                buffer_size_in_out,
                buffer_out.as_ptr() as *mut Void,
            )
        };
        status
    }
}
