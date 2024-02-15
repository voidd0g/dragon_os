use crate::uefi::data_type::basic_type::UnsignedInt32;

#[repr(C)]
pub struct EfiPixelBitmask {
    red_mask: UnsignedInt32,
    green_mask: UnsignedInt32,
    blue_mask: UnsignedInt32,
    ReservedMask: UnsignedInt32,
}
