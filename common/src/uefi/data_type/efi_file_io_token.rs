use crate::uefi::data_type::basic_type::{EFI_EVENT, EfiStatus, UnsignedIntNative, Void};

#[repr(C)]
pub struct EFI_FILE_IO_TOKEN {
    Event: EFI_EVENT,
    Status: EfiStatus,
    BufferSize: UnsignedIntNative,
    Buffer: *mut Void,
}
