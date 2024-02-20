#![no_std]
#![no_main]

mod font;
mod pci;
mod pixel_writer;
mod pointer;
mod util;

use core::{arch::asm, panic::PanicInfo};

use common::{
    argument::{Argument, FrameBufferConfig},
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
    uefi::data_type::basic_type::{UnsignedInt32, UnsignedInt8},
};
use font::font_writer::{FONT_HEIGHT, FONT_WIDTH};

use crate::{
    font::font_writer::FontWriter,
    pci::BusScanner,
    pixel_writer::{draw_rect::DrawRect, pixel_color::PixelColor, put_pixels},
    pointer::PointerWriter,
    util::vector2::Vector2,
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

    macro_rules! output_string {
        ( $color:expr, $pos:expr, $frame_buffer_config:ident, $( $x:expr ),* ) => {
            output_string(&mut [
                $(
                    &mut $x,
                )*
            ], $color, $pos, $frame_buffer_config)
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

    let mut bus_scanner = BusScanner::new();
    let _ = end_with_err! {
        bus_scanner.scan_all_devices()
    };
    let mut height = 0;
    let _ = end_with_err! {
        output_string!(
            PixelColor::new(128, 0, 0),
            Vector2::new(0, height),
            frame_buffer_config,
            bus_scanner.devices_found().len().to_iter_str(IterStrFormat::none()),
            b" devices found.".to_iter_str(IterStrFormat::none())
        )
    };
    height += FONT_HEIGHT;
    for device in bus_scanner.devices_found() {
        let class_codes = device.class_codes();
        let _ = end_with_err! {
            output_string!(
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                frame_buffer_config,
                device.bus().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                device.device().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                device.function().to_iter_str(IterStrFormat::none()),
                b": vendor_id ".to_iter_str(IterStrFormat::none()),
                device.vendor_id().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 4)))),
                b", class_codes ".to_iter_str(IterStrFormat::none()),
                class_codes[0].to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 2)))),
                b"-".to_iter_str(IterStrFormat::none()),
                class_codes[1].to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 2)))),
                b"-".to_iter_str(IterStrFormat::none()),
                class_codes[2].to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 2)))),
                b"-".to_iter_str(IterStrFormat::none()),
                class_codes[3].to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 2)))),
                b", header_type ".to_iter_str(IterStrFormat::none()),
                device.header_type().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 2))))
            )
        };
        height += FONT_HEIGHT;
    }

    let _ = end_with_err! {
        put_pixels(
            frame_buffer_config.pixels_per_scan_line(),
            frame_buffer_config.pixel_format(),
            frame_buffer_config.frame_buffer(),
            PointerWriter::new(Vector2::new(300, 300)),
        )
    };

    end()
}

fn output_string(
    elements: &mut [&mut dyn Iterator<Item = UnsignedInt8>],
    color: PixelColor,
    start_pos: Vector2<UnsignedInt32>,
    frame_buffer_config: &FrameBufferConfig,
) -> Result<(), ()> {
    let mut cur_pos = start_pos;
    let mut elements_iter = elements.iter_mut();
    'a: loop {
        match elements_iter.next() {
            Some(element) => 'b: loop {
                match element.next() {
                    Some(c) => match c {
                        b'\n' => cur_pos = Vector2::new(start_pos.x(), cur_pos.y() + FONT_HEIGHT),
                        c => {
                            let _ = match put_pixels(
                                frame_buffer_config.pixels_per_scan_line(),
                                frame_buffer_config.pixel_format(),
                                frame_buffer_config.frame_buffer(),
                                FontWriter::new(color, cur_pos, c),
                            ) {
                                Ok(res) => res,
                                Err(v) => return Err(v),
                            };
                            cur_pos = Vector2::new(cur_pos.x() + FONT_WIDTH, cur_pos.y())
                        }
                    },
                    None => break 'b (),
                }
            },
            None => break 'a Ok(()),
        }
    }
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
