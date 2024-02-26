use super::basic_type::{EfiEvent, EfiStatus, Void};

#[repr(C)]
pub struct EfiFileIoToken {
    event: EfiEvent,
    status: EfiStatus,
    buffer_size: usize,
    buffer: *mut Void,
}
