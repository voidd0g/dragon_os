use super::{
    basic_type::{EfiPhysicalAddress, UnsignedInt32, UnsignedIntNative},
    efi_graphics_output_mode_information::EfiGraphicsOutputModeInformation,
};

#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
    max_mode: UnsignedInt32,
    mode: UnsignedInt32,
    info: *const EfiGraphicsOutputModeInformation,
    size_of_info: UnsignedIntNative,
    frame_buffer_base: EfiPhysicalAddress,
    frame_buffer_size: UnsignedIntNative,
}

impl EfiGraphicsOutputProtocolMode {
    pub fn info(&self) -> &EfiGraphicsOutputModeInformation {
        unsafe { self.info.as_ref() }.unwrap()
    }

    pub fn frame_buffer_base(&self) -> EfiPhysicalAddress {
        self.frame_buffer_base
    }
    pub fn frame_buffer_size(&self) -> UnsignedIntNative {
        self.frame_buffer_size
    }
}
