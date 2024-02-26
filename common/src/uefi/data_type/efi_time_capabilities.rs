use super::basic_type::Boolean;

#[repr(C)]
pub struct EfiTimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: Boolean,
}
