use crate::uefi::data_type::{
    basic_type::{EFI_GRAPHICS_OUTPUT_BLT_OPERATION, EfiStatus, UnsignedInt32, UnsignedIntNative},
    {
        efi_graphics_output_blt_pixel::EFI_GRAPHICS_OUTPUT_BLT_PIXEL,
        efi_graphics_output_mode_information::EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
        efi_graphics_output_protocol_mode::EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
    },
};

type EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    ModeNumber: UnsignedInt32,
    SizeOfInfoOut: *mut UnsignedIntNative,
    InfoOut: *mut *const EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
) -> EfiStatus;

type EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    ModeNumber: UnsignedInt32,
) -> EfiStatus;

type EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT = unsafe extern "efiapi" fn(
    This: *const EFI_GRAPHICS_OUTPUT_PROTOCOL,
    BltBufferOptional: *mut EFI_GRAPHICS_OUTPUT_BLT_PIXEL,
    BltOperation: EFI_GRAPHICS_OUTPUT_BLT_OPERATION,
    SourceX: UnsignedIntNative,
    SourceY: UnsignedIntNative,
    DestinationX: UnsignedIntNative,
    DestinationY: UnsignedIntNative,
    Width: UnsignedIntNative,
    Height: UnsignedIntNative,
    DeltaOptional: UnsignedIntNative,
) -> EfiStatus;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL {
    QueryMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_QUERY_MODE,
    SetMode: EFI_GRAPHICS_OUTPUT_PROTOCOL_SET_MODE,
    Blt: EFI_GRAPHICS_OUTPUT_PROTOCOL_BLT,
    Mode: *const EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE,
}

impl EFI_GRAPHICS_OUTPUT_PROTOCOL {
    pub fn mode(&self) -> &EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
        unsafe { self.Mode.as_ref() }.unwrap()
    }
}
