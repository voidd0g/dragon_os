use core::slice;

use crate::{
    memory_map::MemoryMap,
    uefi::{
        data_type::basic_type::EfiGraphicsPixelFormat,
        table::efi_runtime_services::EfiRuntimeServices,
    },
};

#[repr(C)]
pub struct Argument {
    frame_buffer_config: *const FrameBufferConfig,
    runtime_services: *const EfiRuntimeServices,
    memory_map: *const MemoryMap,
}

impl Argument {
    pub fn new(
        frame_buffer_config: *const FrameBufferConfig,
        runtime_services: *const EfiRuntimeServices,
        memory_map: *const MemoryMap,
    ) -> Self {
        Self {
            frame_buffer_config,
            runtime_services,
            memory_map,
        }
    }

    pub fn frame_buffer_config(&self) -> &FrameBufferConfig {
        unsafe { self.frame_buffer_config.as_ref() }.unwrap()
    }

    pub fn runtime_services(&self) -> &EfiRuntimeServices {
        unsafe { self.runtime_services.as_ref() }.unwrap()
    }

    pub fn memory_map(&self) -> &MemoryMap {
        unsafe { self.memory_map.as_ref() }.unwrap()
    }
}

#[repr(C)]
pub struct FrameBufferConfig {
    frame_buffer: *mut u8,
    frame_buffer_size: usize,
    pixels_per_scan_line: u32,
    horizontal_resolution: u32,
    vertical_resolution: u32,
    pixel_format: EfiGraphicsPixelFormat,
}

impl FrameBufferConfig {
    pub fn new(
        frame_buffer: *mut u8,
        frame_buffer_size: usize,
        pixels_per_scan_line: u32,
        horizontal_resolution: u32,
        vertical_resolution: u32,
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

    pub fn frame_buffer(&self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.frame_buffer, self.frame_buffer_size) }
    }

    pub fn pixels_per_scan_line(&self) -> u32 {
        self.pixels_per_scan_line
    }

    pub fn horizontal_resolution(&self) -> u32 {
        self.horizontal_resolution
    }

    pub fn vertical_resolution(&self) -> u32 {
        self.vertical_resolution
    }

    pub fn pixel_format(&self) -> EfiGraphicsPixelFormat {
        self.pixel_format
    }
}
