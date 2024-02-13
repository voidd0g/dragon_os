use crate::uefi::data_types::{
    basic_types::{EFI_GRAPHICS_OUTPUT_BLT_OPERATION, EFI_STATUS, UINT32, UINTN},
    structs::{
        efi_graphics_output_blt_pixel::EFI_GRAPHICS_OUTPUT_BLT_PIXEL,
        efi_graphics_output_mode_information::EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
        efi_graphics_output_protocol_mode::EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
    },
};

type EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    ModeNumber: UINT32,
    SizeOfInfoOut: *mut UINTN,
    InfoOut: *mut *const EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
) -> EFI_STATUS;

type EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    ModeNumber: UINT32,
) -> EFI_STATUS;

type EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    BltBufferOptional: *mut EFI_GRAPHICS_OUTPUT_BLT_PIXEL,
    BltOperation: EFI_GRAPHICS_OUTPUT_BLT_OPERATION,
    SourceX: UINTN,
    SourceY: UINTN,
    DestinationX: UINTN,
    DestinationY: UINTN,
    Width: UINTN,
    Height: UINTN,
    DeltaOptional: UINTN,
) -> EFI_STATUS;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL {
    QueryMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE,
    SetMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE,
    Blt: EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT,
    Mode: *const EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
}

#[deny(non_snake_case)]
impl EFI_GRAPHICS_OUTPUT_PROTOCOL {
    pub fn mode(&self) -> &EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
        unsafe { self.Mode.as_ref() }.unwrap()
    }
}
