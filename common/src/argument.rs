use core::slice;

use crate::uefi::data_types::basic_types::{EFI_GRAPHICS_PIXEL_FORMAT, UINT32, UINT8, UINTN};

#[repr(C)]
pub struct Argument {
    frame_buffer_config: *const FrameBufferConfig,
}

#[deny(non_snake_case)]
impl Argument {
    pub fn new(frame_buffer_config: *const FrameBufferConfig) -> Self {
        Self {
            frame_buffer_config,
        }
    }

    pub fn frame_buffer_config(&self) -> &FrameBufferConfig {
        unsafe { self.frame_buffer_config.as_ref() }.unwrap()
    }
}

#[repr(C)]
pub struct FrameBufferConfig {
    frame_buffer: *mut UINT8,
    frame_buffer_size: UINTN,
    pixels_per_scan_line: UINT32,
    horizontal_resolution: UINT32,
    vertical_resolution: UINT32,
    pixel_format: EFI_GRAPHICS_PIXEL_FORMAT,
}

#[deny(non_snake_case)]
impl FrameBufferConfig {
    pub fn new(
        frame_buffer: *mut UINT8,
        frame_buffer_size: UINTN,
        pixels_per_scan_line: UINT32,
        horizontal_resolution: UINT32,
        vertical_resolution: UINT32,
        pixel_format: EFI_GRAPHICS_PIXEL_FORMAT,
    ) -> Self {
        Self {
            frame_buffer,
            frame_buffer_size,
            pixels_per_scan_line,
            horizontal_resolution,
            vertical_resolution,
            pixel_format,
        }
    }
    
    pub fn frame_buffer(&self) -> &mut[UINT8] {
        unsafe {
            slice::from_raw_parts_mut(self.frame_buffer, self.frame_buffer_size)
        }
    }

    pub fn pixels_per_scan_line(&self) -> UINT32 {
        self.pixels_per_scan_line
    }

    pub fn horizontal_resolution(&self) -> UINT32 {
        self.horizontal_resolution
    }

    pub fn vertical_resolution(&self) -> UINT32 {
        self.vertical_resolution
    }

    pub fn pixel_format(&self) -> EFI_GRAPHICS_PIXEL_FORMAT {
        self.pixel_format
    }
}
