use crate::uefi::data_type::basic_type::{EFI_GRAPHICS_PIXEL_FORMAT, UnsignedInt32};

use super::efi_pixel_bitmask::EfiPixelBitmask;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    Version: UnsignedInt32,
    HorizontalResolution: UnsignedInt32,
    VerticalResolution: UnsignedInt32,
    PixelFormat: EFI_GRAPHICS_PIXEL_FORMAT,
    PixelInformation: EfiPixelBitmask,
    PixelsPerScanLine: UnsignedInt32,
}

impl EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
    pub fn horizontal_resolution(&self) -> UnsignedInt32 {
        self.HorizontalResolution
    }
    pub fn vertical_resolution(&self) -> UnsignedInt32 {
        self.VerticalResolution
    }
    pub fn pixel_format(&self) -> EFI_GRAPHICS_PIXEL_FORMAT {
        self.PixelFormat
    }

    pub fn pixels_per_scan_line(&self) -> UnsignedInt32 {
        self.PixelsPerScanLine
    }
}
