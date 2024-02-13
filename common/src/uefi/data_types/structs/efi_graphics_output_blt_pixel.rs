use crate::uefi::data_types::basic_types::UINT32;

#[repr(C)]
pub struct EFI_GRAPHICS_OUTPUT_BLT_PIXEL {
    Blue: UINT32,
    Green: UINT32,
    Red: UINT32,
    Reserved: UINT32,
}
