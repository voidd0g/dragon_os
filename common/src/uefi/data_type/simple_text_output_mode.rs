use super::basic_type::Boolean;

#[repr(C)]
pub struct SimpleTextOutputMode {
    max_mode: i32,
    attribute: i32,
    cursor_column: i32,
    cursor_row: i32,
    cursor_visible: Boolean,
}
