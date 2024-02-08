use crate::uefi::data_types::{basic_types::{BOOLEAN, CHAR16, EFI_STATUS, UINTN}, structs::simple_text_output_mode::SIMPLE_TEXT_OUTPUT_MODE};

type EFI_TEXT_RESET = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: BOOLEAN) -> EFI_STATUS;
type EFI_TEXT_STRING = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS;
type EFI_TEXT_TEST_STRING = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS;
type EFI_TEXT_QUERY_MODE = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: UINTN, ColumnsOut: *mut UINTN, Rows: *mut UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_MODE = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ModeNumber: UINTN) -> EFI_STATUS;
type EFI_TEXT_SET_ATTRIBUTE = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Attribute: UINTN) -> EFI_STATUS;
type EFI_TEXT_CLEAR_SCREEN = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS;
type EFI_TEXT_SET_CURSOR_POSITION = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Column: UINTN, Row: UINTN) -> EFI_STATUS;
type EFI_TEXT_ENABLE_CURSOR = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, Visible: BOOLEAN) -> EFI_STATUS;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/12_Protocols_Console_Support.html#efi-simple-text-output-protocol
#[repr(C)]
pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: EFI_TEXT_RESET,
    OutputString: EFI_TEXT_STRING,
    TestString: EFI_TEXT_TEST_STRING,
    QueryMode: EFI_TEXT_QUERY_MODE,
    SetMode: EFI_TEXT_SET_MODE,
    SetAttribute: EFI_TEXT_SET_ATTRIBUTE,
    ClearScreen: EFI_TEXT_CLEAR_SCREEN,
    SetCursorPosition: EFI_TEXT_SET_CURSOR_POSITION,
    EnableCursor: EFI_TEXT_ENABLE_CURSOR,
    Mode: *const SIMPLE_TEXT_OUTPUT_MODE,
}

#[deny(non_snake_case)]
impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub fn reset(&self, extended_verification: bool) -> EFI_STATUS {
        (self.Reset)(self, if extended_verification { 1u8 } else { 0u8 })
    }

    pub fn output_string(&self, string: &[CHAR16]) -> EFI_STATUS {
        (self.OutputString)(self, string.as_ptr())
    }
}