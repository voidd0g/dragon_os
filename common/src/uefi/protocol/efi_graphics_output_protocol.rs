use crate::uefi::data_type::{
    basic_type::{EfiGraphicsOutputBltOperation, EfiStatus, UnsignedInt32, UnsignedIntNative},
    {
        efi_graphics_output_blt_pixel::EfiGraphicsOutputBltPixel,
        efi_graphics_output_mode_information::EfiGraphicsOutputModeInformation,
        efi_graphics_output_protocol_mode::EfiGraphicsOutputProtocolMode,
    },
};

type EfiGraphicsOutputProtocolQueryMode = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    mode_number: UnsignedInt32,
    size_of_info_out: *mut UnsignedIntNative,
    info_out: *mut *const EfiGraphicsOutputModeInformation,
) -> EfiStatus;

type EfiGraphicsOutputProtocolSetMode = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    mode_number: UnsignedInt32,
) -> EfiStatus;

type EfiGraphicsOutputProtocolBlt = unsafe extern "efiapi" fn(
    this: *const EfiGraphicsOutputProtocol,
    blt_buffer_optional: *mut EfiGraphicsOutputBltPixel,
    blt_operation: EfiGraphicsOutputBltOperation,
    source_x: UnsignedIntNative,
    source_y: UnsignedIntNative,
    destination_x: UnsignedIntNative,
    destination_y: UnsignedIntNative,
    width: UnsignedIntNative,
    height: UnsignedIntNative,
    delta_optional: UnsignedIntNative,
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
