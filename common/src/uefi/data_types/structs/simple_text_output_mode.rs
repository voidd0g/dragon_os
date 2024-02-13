use crate::uefi::data_types::basic_types::{BOOLEAN, INT32};

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    MaxMode: INT32,
    Attribute: INT32,
    CursorColumn: INT32,
    CursorRow: INT32,
    CursorVisible: BOOLEAN,
}
