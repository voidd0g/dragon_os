use crate::uefi::data_type::basic_type::{Boolean, Int32};

#[repr(C)]
pub struct SIMPLE_TEXT_OUTPUT_MODE {
    MaxMode: Int32,
    Attribute: Int32,
    CursorColumn: Int32,
    CursorRow: Int32,
    CursorVisible: Boolean,
}
