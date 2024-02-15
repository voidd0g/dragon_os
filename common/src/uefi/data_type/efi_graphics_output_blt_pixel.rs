use super::basic_type::UnsignedInt32;

#[repr(C)]
pub struct EfiGraphicsOutputBltPixel {
    blue: UnsignedInt32,
    green: UnsignedInt32,
    red: UnsignedInt32,
    reserved: UnsignedInt32,
}
