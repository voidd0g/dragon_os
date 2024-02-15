use core::slice;

use crate::uefi::data_type::basic_type::{
    EfiGraphicsPixelFormat, UnsignedInt32, UnsignedInt8, UnsignedIntNative,
};

#[repr(C)]
pub struct Argument {
    frame_buffer_config: *const FrameBufferConfig,
}

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
    frame_buffer: *mut UnsignedInt8,
    frame_buffer_size: UnsignedIntNative,
    pixels_per_scan_line: UnsignedInt32,
    horizontal_resolution: UnsignedInt32,
    vertical_resolution: UnsignedInt32,
    pixel_format: EfiGraphicsPixelFormat,
}

impl FrameBufferConfig {
    pub fn new(
        frame_buffer: *mut UnsignedInt8,
        frame_buffer_size: UnsignedIntNative,
        pixels_per_scan_line: UnsignedInt32,
        horizontal_resolution: UnsignedInt32,
        vertical_resolution: UnsignedInt32,
        pixel_format: EfiGraphicsPixelFormat,
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

    pub fn frame_buffer(&self) -> &mut [UnsignedInt8] {
        unsafe { slice::from_raw_parts_mut(self.frame_buffer, self.frame_buffer_size) }
    }

    pub fn pixels_per_scan_line(&self) -> UnsignedInt32 {
        self.pixels_per_scan_line
    }

    pub fn horizontal_resolution(&self) -> UnsignedInt32 {
        self.horizontal_resolution
    }

    pub fn vertical_resolution(&self) -> UnsignedInt32 {
        self.vertical_resolution
    }

    pub fn pixel_format(&self) -> EfiGraphicsPixelFormat {
        self.pixel_format
    }
}
