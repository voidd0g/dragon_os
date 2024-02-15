use crate::uefi::data_type::basic_type::{EFI_PHYSICAL_ADDRESS, UnsignedInt32, UnsignedIntNative};

use super::efi_graphics_output_mode_information::EFI_GRAPHICS_OUTPUT_MODE_INFORMATION;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    MaxMode: UnsignedInt32,
    Mode: UnsignedInt32,
    Info: *const EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    SizeOfInfo: UnsignedIntNative,
    FrameBufferBase: EFI_PHYSICAL_ADDRESS,
    FrameBufferSize: UnsignedIntNative,
}

impl EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    pub fn info(&self) -> &EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
        unsafe { self.Info.as_ref() }.unwrap()
    }

    pub fn frame_buffer_base(&self) -> EFI_PHYSICAL_ADDRESS {
        self.FrameBufferBase
    }
    pub fn frame_buffer_size(&self) -> UnsignedIntNative {
        self.FrameBufferSize
    }
}
