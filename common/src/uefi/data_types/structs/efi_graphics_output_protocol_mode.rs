use crate::uefi::data_types::basic_types::{EFI_PHYSICAL_ADDRESS, UINT32, UINTN};

use super::efi_graphics_output_mode_information::EFI_GRAPHICS_OUTPUT_MODE_INFORMATION;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    MaxMode: UINT32,
    Mode: UINT32,
    Info: *const EFI_GRAPHICS_OUTPUT_MODE_INFORMATION,
    SizeOfInfo: UINTN,
    FrameBufferBase: EFI_PHYSICAL_ADDRESS,
    FrameBufferSize: UINTN,
}

#[deny(non_snake_case)]
impl EFI_GRAPHICS_OUTPUT_PROTOCOL_MODE {
    pub fn info(&self) -> &EFI_GRAPHICS_OUTPUT_MODE_INFORMATION {
        unsafe {
            self.Info.as_ref()
        }.unwrap()
    }

    pub fn frame_buffer_base(&self) -> EFI_PHYSICAL_ADDRESS {
        self.FrameBufferBase
    }
    pub fn frame_buffer_size(&self) -> UINTN {
        self.FrameBufferSize
    }
}
