use crate::uefi::data_type::{
    basic_type::{Boolean, Char16, EfiStatus, UnsignedIntNative},
    simple_text_output_mode::SimpleTextOutputMode,
};

type EfiTextReset = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    extended_verification: Boolean,
) -> EfiStatus;
type EfiTextString = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    string: *const Char16,
) -> EfiStatus;
type EfiTextTestString = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    string: *const Char16,
) -> EfiStatus;
type EfiTextQueryMode = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    mode_number: UnsignedIntNative,
    columns_out: *mut UnsignedIntNative,
    rows: *mut UnsignedIntNative,
) -> EfiStatus;
type EfiTextSetMode = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    mode_number: UnsignedIntNative,
) -> EfiStatus;
type EfiTextSetAttribute = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    attribute: UnsignedIntNative,
) -> EfiStatus;
type EfiTextClearScreen =
    unsafe extern "efiapi" fn(this: *const EfiSimpleTextOutputProtocol) -> EfiStatus;
type EfiTextSetCursorPosition = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    column: UnsignedIntNative,
    row: UnsignedIntNative,
) -> EfiStatus;
type EfiTextEnableCursor = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    visible: Boolean,
) -> EfiStatus;

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    reset: EfiTextReset,
    output_string: EfiTextString,
    test_string: EfiTextTestString,
    query_mode: EfiTextQueryMode,
    set_mode: EfiTextSetMode,
    set_attribute: EfiTextSetAttribute,
    clear_screen: EfiTextClearScreen,
    set_cursor_position: EfiTextSetCursorPosition,
    enable_cursor: EfiTextEnableCursor,
    mode: *const SimpleTextOutputMode,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        unsafe { (self.reset)(self, if extended_verification { 1u8 } else { 0u8 }) }
    }

    pub fn output_string(&self, string: &[Char16]) -> EfiStatus {
        unsafe { (self.output_string)(self, string.as_ptr()) }
    }
}
