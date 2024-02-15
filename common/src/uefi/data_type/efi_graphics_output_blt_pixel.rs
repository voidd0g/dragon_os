use crate::uefi::data_type::basic_type::UnsignedInt32;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_BLT_PIXEL {
    Blue: UnsignedInt32,
    Green: UnsignedInt32,
    Red: UnsignedInt32,
    Reserved: UnsignedInt32,
}
