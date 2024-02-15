#![no_std]
#![no_main]

use core::{arch::asm, fmt::Debug, panic::PanicInfo};

use common::{
    argument::{Argument, FrameBufferConfig},
    uefi::{
        constant::efi_graphics_pixel_format::{
            PixelBlueGreenRedReserved8BitPerColor, PixelRedGreenBlueReserved8BitPerColor,
        },
        data_type::basic_type::{UINT8, UINTN},
    },
};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(arg: *const Argument) -> ! {
    let arg = unsafe { arg.as_ref() }.unwrap();
    let frame_buffer_config = arg.frame_buffer_config();
    for y in 0..frame_buffer_config.vertical_resolution() {
        for x in 0..frame_buffer_config.horizontal_resolution() {
            if 100 <= x && x < 300 && 100 <= y && y < 500 {
                match put_pixel(
                    frame_buffer_config,
                    PixelColor {
                        red: 255,
                        green: 255,
                        blue: 0,
                    },
                    Vector2(x as UINTN, y as UINTN),
                ) {
                    Ok(_) => (),
                    Err(_) => end(),
                }
            } else {
                match put_pixel(
                    frame_buffer_config,
                    PixelColor {
                        red: 0,
                        green: 255,
                        blue: 255,
                    },
                    Vector2(x as UINTN, y as UINTN),
                ) {
                    Ok(_) => (),
                    Err(_) => end(),
                }
            }
        }
    }

    end()
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn put_pixel(
    frame_buffer_config: &FrameBufferConfig,
    color: PixelColor,
    pos: Vector2<UINTN>,
) -> Result<(), ()> {
    match frame_buffer_config.pixel_format() {
        PixelRedGreenBlueReserved8BitPerColor => {
            let pixel_start_pos =
                frame_buffer_config.pixels_per_scan_line() as UINTN * pos.1 + pos.0;
            let mut iter = frame_buffer_config.frame_buffer().iter_mut();
            *iter.nth(pixel_start_pos * 4).unwrap() = color.red;
            *iter.next().unwrap() = color.green;
            *iter.next().unwrap() = color.blue;
            Ok(())
        }
        PixelBlueGreenRedReserved8BitPerColor => {
            let pixel_start_pos =
                frame_buffer_config.pixels_per_scan_line() as UINTN * pos.1 + pos.0;
            let mut iter = frame_buffer_config.frame_buffer().iter_mut();
            *iter.nth(pixel_start_pos * 4).unwrap() = color.blue;
            *iter.next().unwrap() = color.green;
            *iter.next().unwrap() = color.red;
            Ok(())
        }
        _ => Err(()),
    }
}

#[derive(Clone, Copy, Debug)]
struct PixelColor {
    pub red: UINT8,
    pub green: UINT8,
    pub blue: UINT8,
}

#[derive(Clone, Copy, Debug)]
struct Vector2<T: Clone + Copy + Debug>(T, T);
