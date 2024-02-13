pub type PixelFormat = usize;

pub const PIXEL_RGB_RESV_8_BIT_PER_COLOR: PixelFormat = 0;
pub const PIXEL_BGR_RESV_8_BIT_PER_COLOR: PixelFormat = 0;

#[repr(C)]
pub struct FrameBufferConfig<'a> {
	frame_buffer: &'a [u8],
	pixels_per_scan_line: 
}