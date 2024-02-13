use crate::uefi::data_types::basic_types::{EFI_GRAPHICS_PIXEL_FORMAT, UINT32};

use super::efi_pixel_bitmask::EFI_PIXEL_BITMASK;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    Version: UINT32,
    HorizontalResolution: UINT32,
    VerticalResolution: UINT32,
    PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
    PixelInformation: EFI_PIXEL_BITMASK,
    PixelsPerScanLine: UINT32,
}
