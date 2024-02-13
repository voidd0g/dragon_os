use crate::uefi::data_types::basic_types::UINT32;

#[repr(C)]
pub struct EFI_PIXEL_BITMASK {
    RedMask: UINT32,
    GreenMask: UINT32,
    BlueMask: UINT32,
    ReservedMask: UINT32,
}
