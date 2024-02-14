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

#[deny(non_snake_case)]
impl EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    pub fn horizontal_resolution(&self) -> UINT32 {
        self.HorizontalResolution
    }
    pub fn vertical_resolution(&self) -> UINT32 {
        self.VerticalResolution
    }
    pub fn pixel_format(&self) -> EFI_GRAPHICS_PIXEL_FORMAT {
        self.PixelFormat
    }
    
    pub fn pixels_per_scan_line(&self) -> UINT32 {
        self.PixelsPerScanLine
    }
}