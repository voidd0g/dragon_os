use core::ptr::null;

use crate::{uefi::data_types::{basic_types::{CHAR16, EFI_GUID, EFI_STATUS, UINT64, UINT8, UINTN, VOID}, structs::efi_file_io_token::EFI_FILE_IO_TOKEN}, utils::from_byte_slice::FromByteSlice};

type EFI_FILE_OPEN = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, NewHandleOut: *mut *const EFI_FILE_PROTOCOL, FileName: *const CHAR16, OpenMode: UINT64, Attributes: UINT64) -> EFI_STATUS;
type EFI_FILE_CLOSE = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL) -> EFI_STATUS;
type EFI_FILE_DELETE = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL) -> EFI_STATUS;
type EFI_FILE_READ = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, BufferSizeInOut: *mut UINTN, BufferOut: *mut VOID) -> EFI_STATUS;
type EFI_FILE_WRITE = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, BufferSizeInOut: *mut UINTN, Buffer: *const VOID) -> EFI_STATUS;
type EFI_FILE_GET_POSITION = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, PositionOut: *mut UINT64) -> EFI_STATUS;
type EFI_FILE_SET_POSITION = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, Position: UINT64) -> EFI_STATUS;
type EFI_FILE_GET_INFO = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, InformationType: *const EFI_GUID, BufferSizeInOut: *mut UINTN, BufferOut: *mut VOID) -> EFI_STATUS;
type EFI_FILE_SET_INFO = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, InformationType: *const EFI_GUID, BufferSize: UINTN, Buffer: *const VOID) -> EFI_STATUS;
type EFI_FILE_FLUSH = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL) -> EFI_STATUS;
type EFI_FILE_OPEN_EX = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, NewHandleOut: *mut *const EFI_FILE_PROTOCOL, FileName: *const CHAR16, OpenMode: UINT64, Attributes: UINT64, TokenInOut: *mut EFI_FILE_IO_TOKEN) -> EFI_STATUS;
type EFI_FILE_READ_EX = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, TokenInOut: *mut EFI_FILE_IO_TOKEN) -> EFI_STATUS;
type EFI_FILE_WRITE_EX = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, TokenInOut: *mut EFI_FILE_IO_TOKEN) -> EFI_STATUS;
type EFI_FILE_FLUSH_EX = unsafe extern "efiapi" fn (This: *const EFI_FILE_PROTOCOL, TokenInOut: *mut EFI_FILE_IO_TOKEN) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_FILE_PROTOCOL {
    Revision: UINT64,
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

#[deny(non_snake_case)]
impl EFI_FILE_PROTOCOL {

    pub fn open(&self, file_name: &[CHAR16], open_mode: UINT64, attributes: UINT64) -> (EFI_STATUS, &EFI_FILE_PROTOCOL) {
        let mut new_handle_out = null();
        let status = unsafe {
            (self.Open)(self, &mut new_handle_out, file_name.as_ptr(), open_mode, attributes)
        };
        (status, unsafe {
            new_handle_out.as_ref()
        }.unwrap())
    }
    pub fn close(&self) -> EFI_STATUS {
        let status = unsafe {
            (self.Close)(self)
        };
        status
    }

    pub fn write(&self, buffer_size_in_out: &mut UINTN, buffer: &[UINT8]) -> EFI_STATUS {
        let status = unsafe { 
            (self.Write)(self, buffer_size_in_out, buffer.as_ptr() as *const VOID)
        };
        status
    }
}

#[deny(non_snake_case)]
impl FromByteSlice for EFI_FILE_PROTOCOL {
    fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
        let (revision, bs) = UINT64::from_byte_slice(bs);
        let (open, bs) = <*const EFI_FILE_OPEN>::from_byte_slice(bs);
        let (cloase, bs) = <*const EFI_FILE_CLOSE>::from_byte_slice(bs);
        let (delete, bs) = <*const EFI_FILE_DELETE>::from_byte_slice(bs);
        let (read, bs) = <*const EFI_FILE_READ>::from_byte_slice(bs);
        let (write, bs) = <*const EFI_FILE_WRITE>::from_byte_slice(bs);
        let (get_position, bs) = <*const EFI_FILE_GET_POSITION>::from_byte_slice(bs);
        let (set_position, bs) = <*const EFI_FILE_SET_POSITION>::from_byte_slice(bs);
        let (get_info, bs) = <*const EFI_FILE_GET_INFO>::from_byte_slice(bs);
        let (set_info, bs) = <*const EFI_FILE_SET_INFO>::from_byte_slice(bs);
        let (flush, bs) = <*const EFI_FILE_FLUSH>::from_byte_slice(bs);
        let (open_ex, bs) = <*const EFI_FILE_OPEN_EX>::from_byte_slice(bs);
        let (read_ex, bs) = <*const EFI_FILE_READ_EX>::from_byte_slice(bs);
        let (write_ex, bs) = <*const EFI_FILE_WRITE_EX>::from_byte_slice(bs);
        let (flush_ex, bs) = <*const EFI_FILE_FLUSH_EX>::from_byte_slice(bs);

        (Self { 
            Revision: revision, 
            Open: unsafe { open.read() }, 
            Close: unsafe { cloase.read() }, 
            Delete: unsafe { delete.read() }, 
            Read: unsafe { read.read() }, 
            Write: unsafe { write.read() }, 
            GetPosition: unsafe { get_position.read() }, 
            SetPosition: unsafe { set_position.read() },
            GetInfo: unsafe { get_info.read() }, 
            SetInfo: unsafe { set_info.read() },
            Flush: unsafe { flush.read() },
            OpenEx: unsafe { open_ex.read() },
            ReadEx: unsafe { read_ex.read() },
            WriteEx: unsafe { write_ex.read() },
            FlushEx: unsafe { flush_ex.read() },
        }, bs)
    }
}