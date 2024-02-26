use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::{
        basic_type::{Boolean, EfiStatus},
        simple_text_output_mode::SimpleTextOutputMode,
    },
};

type EfiTextReset = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    extended_verification: Boolean,
) -> EfiStatus;
type EfiTextString = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    string: *const u16,
) -> EfiStatus;
type EfiTextTestString = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    string: *const u16,
) -> EfiStatus;
type EfiTextQueryMode = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    mode_number: usize,
    columns_out: *mut usize,
    rows: *mut usize,
) -> EfiStatus;
type EfiTextSetMode = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    mode_number: usize,
) -> EfiStatus;
type EfiTextSetAttribute = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    attribute: usize,
) -> EfiStatus;
type EfiTextClearScreen =
    unsafe extern "efiapi" fn(this: *const EfiSimpleTextOutputProtocol) -> EfiStatus;
type EfiTextSetCursorPosition = unsafe extern "efiapi" fn(
    this: *const EfiSimpleTextOutputProtocol,
    column: usize,
    row: usize,
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
    pub fn reset(&self, extended_verification: bool) -> Result<(), EfiStatus> {
        let status = unsafe { (self.reset)(self, if extended_verification { 1u8 } else { 0u8 }) };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }

    pub fn output_string(&self, string: &[u16]) -> Result<(), EfiStatus> {
        let status = unsafe { (self.output_string)(self, string.as_ptr()) };
        match status {
            EFI_SUCCESS => Ok(()),
            v => Err(v),
        }
    }
}
