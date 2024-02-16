#![no_std]
#![no_main]

mod font;
mod pixel_writer;
mod util;

use core::{arch::asm, panic::PanicInfo};

use common::{argument::Argument, uefi::data_type::basic_type::UnsignedInt8};

use crate::{
    font::font_writer::FontWriter, pixel_writer::{draw_rect::DrawRect, pixel_color::PixelColor, put_pixels}, util::vector2::Vector2
};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(arg: *const Argument) -> ! {
    let arg = unsafe { arg.as_ref() }.unwrap();

    macro_rules! end_with_err {
        ( $e:expr ) => {
            match $e {
                Ok(res) => res,
                Err(_) => end(),
            }
        };
    }

    let frame_buffer_config = arg.frame_buffer_config();

    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            DrawRect::new(PixelColor::new(0, 255, 128), Vector2::new(0, 0), Vector2::new(frame_buffer_config.horizontal_resolution(), frame_buffer_config.vertical_resolution())),
        )
    };

    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            DrawRect::new(PixelColor::new(0, 128, 255), Vector2::new(100, 100), Vector2::new(300, 500) ),
        )
    };

    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            FontWriter::new(PixelColor::new(0, 0, 0), Vector2::new(200, 200), 'A' as UnsignedInt8),
        )
    };
    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            FontWriter::new(PixelColor::new(128, 0, 0), Vector2::new(208, 200), 'B' as UnsignedInt8),
        )
    };
    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            FontWriter::new(PixelColor::new(255, 0, 0), Vector2::new(216, 200), 'C' as UnsignedInt8),
        )
    };

    end()
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
