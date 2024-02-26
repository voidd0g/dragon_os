#[repr(C)]
pub struct EfiGraphicsOutputBltPixel {
    blue: u32,
    green: u32,
    red: u32,
    reserved: u32,
}
