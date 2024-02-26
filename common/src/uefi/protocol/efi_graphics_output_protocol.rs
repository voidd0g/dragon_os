use crate::uefi::data_type::{
    basic_type::{EfiGraphicsOutputBltOperation, EfiStatus},
    {
        efi_graphics_output_blt_pixel::EfiGraphicsOutputBltPixel,
        efi_graphics_output_mode_information::EfiGraphicsOutputModeInformation,
        efi_graphics_output_protocol_mode::EfiGraphicsOutputProtocolMode,
    },
};

type EfiGraphicsOutputProtocolQueryMode = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    mode_number: u32,
    size_of_info_out: *mut usize,
    info_out: *mut *const EfiGraphicsOutputModeInformation,
) -> EfiStatus;

type EfiGraphicsOutputProtocolSetMode = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    mode_number: u32,
) -> EfiStatus;

type EfiGraphicsOutputProtocolBlt = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    blt_buffer_optional: *mut EfiGraphicsOutputBltPixel,
    blt_operation: EfiGraphicsOutputBltOperation,
    source_x: usize,
    source_y: usize,
    destination_x: usize,
    destination_y: usize,
    width: usize,
    height: usize,
    delta_optional: usize,
) -> EfiStatus;

#[repr(C)]
pub struct EfiGraphicsOutputProtocol {
    query_mode: EfiGraphicsOutputProtocolQueryMode,
    set_mode: EfiGraphicsOutputProtocolSetMode,
    blt: EfiGraphicsOutputProtocolBlt,
    mode: *const EfiGraphicsOutputProtocolMode,
}

impl EfiGraphicsOutputProtocol {
    pub fn mode(&self) -> &EfiGraphicsOutputProtocolMode {
        unsafe { self.mode.as_ref() }.unwrap()
    }
}
