use super::basic_type::{Boolean, Int32};

#[repr(C)]
pub struct SimpleTextOutputMode {
    max_mode: Int32,
    attribute: Int32,
    cursor_column: Int32,
    cursor_row: Int32,
    cursor_visible: Boolean,
}
