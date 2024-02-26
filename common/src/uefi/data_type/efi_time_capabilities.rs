use super::basic_type::{Boolean, UnsignedInt32};

#[repr(C)]
pub struct EfiTimeCapabilities {
    resolution: UnsignedInt32,
    accuracy: UnsignedInt32,
    sets_to_zero: Boolean,
}
