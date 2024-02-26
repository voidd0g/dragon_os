use super::{
    basic_type::EfiPhysicalAddress,
    efi_graphics_output_mode_information::EfiGraphicsOutputModeInformation,
};

#[repr(C)]
pub struct EfiGraphicsOutputProtocolMode {
    max_mode: u32,
    mode: u32,
    info: *const EfiGraphicsOutputModeInformation,
    size_of_info: usize,
    frame_buffer_base: EfiPhysicalAddress,
    frame_buffer_size: usize,
}

impl EfiGraphicsOutputProtocolMode {
    pub fn info(&self) -> &EfiGraphicsOutputModeInformation {
        unsafe { self.info.as_ref() }.unwrap()
    }

    pub fn frame_buffer_base(&self) -> EfiPhysicalAddress {
        self.frame_buffer_base
    }
    pub fn frame_buffer_size(&self) -> usize {
        self.frame_buffer_size
    }
}
