use super::basic_type::Boolean;

#[repr(C)]
pub struct EfiTimeCapabilities {
    resolution: u32,
    accuracy: u32,
    sets_to_zero: Boolean,
}

impl EfiTimeCapabilities {
    pub fn new(resolution: u32, accuracy: u32, sets_to_zero: Boolean) -> Self {
        Self {
            resolution,
            accuracy,
            sets_to_zero,
        }
    }
}
