use crate::uefi::data_type::{
    basic_type::{Boolean, Char16, EfiStatus, UnsignedIntNative},
    simple_text_output_mode::SIMPLE_TEXT_OUTPUT_MODE,
};

type EfiTextReset = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    ExtendedVerification: Boolean,
) -> EfiStatus;
type EfiTextString = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    String: *const Char16,
) -> EfiStatus;
type EfiTextTestString = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    String: *const Char16,
) -> EfiStatus;
type EfiTextQueryMode = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    ModeNumber: UnsignedIntNative,
    ColumnsOut: *mut UnsignedIntNative,
    Rows: *mut UnsignedIntNative,
) -> EfiStatus;
type EfiTextSetMode = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    ModeNumber: UnsignedIntNative,
) -> EfiStatus;
type EfiTextSetAttribute = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    Attribute: UnsignedIntNative,
) -> EfiStatus;
type EfiTextClearScreen =
    unsafe extern "efiapi" fn(This: *const EfiSimpleTextOutputProtocol) -> EfiStatus;
type EfiTextSetCursorPosition = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    Column: UnsignedIntNative,
    Row: UnsignedIntNative,
) -> EfiStatus;
type EfiTextEnableCursor = unsafe extern "efiapi" fn(
    This: *const EfiSimpleTextOutputProtocol,
    Visible: Boolean,
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
    mode: *const SIMPLE_TEXT_OUTPUT_MODE,
}

impl EfiSimpleTextOutputProtocol {
    pub fn reset(&self, extended_verification: bool) -> EfiStatus {
        unsafe { (self.reset)(self, if extended_verification { 1u8 } else { 0u8 }) }
    }

    pub fn output_string(&self, string: &[Char16]) -> EfiStatus {
        unsafe { (self.output_string)(self, string.as_ptr()) }
    }
}
