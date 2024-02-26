use super::{basic_type::EfiGraphicsPixelFormat, efi_pixel_bitmask::EfiPixelBitmask};

#[repr(C)]
pub struct EfiGraphicsOutputModeInformation {
    version: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: EfiGraphicsPixelFormat,
    pixel_information: EfiPixelBitmask,
    pixels_per_scan_line: u32,
}

impl EfiGraphicsOutputModeInformation {
    pub fn horizontal_resolution(&self) -> u32 {
        self.horizontal_resolution
    }
    pub fn vertical_resolution(&self) -> u32 {
        self.vertical_resolution
    }
    pub fn pixel_format(&self) -> EfiGraphicsPixelFormat {
        self.pixel_format
    }

    pub fn pixels_per_scan_line(&self) -> u32 {
        self.pixels_per_scan_line
    }
}
