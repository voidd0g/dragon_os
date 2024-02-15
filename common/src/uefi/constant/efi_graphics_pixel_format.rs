use crate::uefi::data_type::basic_type::EfiGraphicsPixelFormat;

pub const PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR: EfiGraphicsPixelFormat = 0;
pub const PIXEL_BLUE_GREEN_RED_RESERVED8_BIT_PER_COLOR: EfiGraphicsPixelFormat = 1;
pub const PIXEL_BIT_MASK: EfiGraphicsPixelFormat = 2;
pub const PIXEL_BLT_ONLY: EfiGraphicsPixelFormat = 3;
pub const PIXEL_FORMAT_MAX: EfiGraphicsPixelFormat = 4;
