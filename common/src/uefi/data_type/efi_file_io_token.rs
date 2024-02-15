use super::basic_type::{EfiEvent, EfiStatus, UnsignedIntNative, Void};

#[repr(C)]
pub struct EfiFileIoToken {
    event: EfiEvent,
    status: EfiStatus,
    buffer_size: UnsignedIntNative,
    buffer: *mut Void,
}
