use super::basic_type::{EfiGraphicsPixelFormat, UnsignedInt32};

use super::efi_pixel_bitmask::EfiPixelBitmask;

#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
    version: UnsignedInt32,
    horizontal_resolution: UnsignedInt32,
    vertical_resolution: UnsignedInt32,
    pixel_format: EfiGraphicsPixelFormat,
    pixel_information: EfiPixelBitmask,
    pixels_per_scan_line: UnsignedInt32,
}

impl EfiGraphicsOutputModeInformation {
    pub fn horizontal_resolution(&self) -> UnsignedInt32 {
        self.horizontal_resolution
    }
    pub fn vertical_resolution(&self) -> UnsignedInt32 {
        self.vertical_resolution
    }
    pub fn pixel_format(&self) -> EfiGraphicsPixelFormat {
        self.pixel_format
    }

    pub fn pixels_per_scan_line(&self) -> UnsignedInt32 {
        self.pixels_per_scan_line
    }
}
