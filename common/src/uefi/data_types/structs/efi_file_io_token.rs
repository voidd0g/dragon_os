use crate::uefi::data_types::basic_types::{EFI_EVENT, EFI_STATUS, UINTN, VOID};

#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
    Event: EFI_EVENT,
    Status: EFI_STATUS,
    BufferSize: UINTN,
    Buffer: *mut VOID,
}