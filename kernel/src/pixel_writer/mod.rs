pub mod draw_rect;
pub mod pixel_color;

use common::uefi::{
    constant::efi_graphics_pixel_format::{
        PIXEL_BLUE_GREEN_RED_RESERVED8_BIT_PER_COLOR, PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR,
    },
    data_type::basic_type::{
        EfiGraphicsPixelFormat, UnsignedInt32, UnsignedInt8, UnsignedIntNative,
    },
};
use pixel_color::PixelColor;

use crate::util::vector2::Vector2;

pub trait PixelWriter<T: PixelLineWriter>:
    Iterator<Item = (T, Vector2<UnsignedIntNative>)>
{
}
pub trait PixelLineWriter: Iterator<Item = Option<PixelColor>> {}

pub fn put_pixel(
    pixels_per_scan_line: UnsignedInt32,
    pixel_format: EfiGraphicsPixelFormat,
    frame_buffer: &mut [UnsignedInt8],
    color: PixelColor,
    pos: Vector2<UnsignedIntNative>,
) -> Result<(), ()> {
    match pixel_format {
        PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR => {
            let pixel_start_pos = pixels_per_scan_line as UnsignedIntNative * pos.y() + pos.x();
            let mut iter = frame_buffer.iter_mut();
            match (
                iter.nth(pixel_start_pos * 4),
                iter.next(),
                iter.next(),
                iter.next(),
            ) {
                (Some(red_target), Some(green_target), Some(blue_target), Some(_)) => {
                    *red_target = color.red();
                    *green_target = color.green();
                    *blue_target = color.blue();
                    Ok(())
                }
                (None, _, _, _) => Ok(()),
                _ => Err(()),
            }
        }
        PIXEL_BLUE_GREEN_RED_RESERVED8_BIT_PER_COLOR => {
            let pixel_start_pos = pixels_per_scan_line as UnsignedIntNative * pos.y() + pos.x();
            let mut iter = frame_buffer.iter_mut();
            match (
                iter.nth(pixel_start_pos * 4),
                iter.next(),
                iter.next(),
                iter.next(),
            ) {
                (Some(blue_target), Some(green_target), Some(red_target), Some(_)) => {
                    *red_target = color.red();
                    *green_target = color.green();
                    *blue_target = color.blue();
                    Ok(())
                }
                (None, _, _, _) => Ok(()),
                _ => Err(()),
            }
        }
        _ => Err(()),
    }
}

pub fn put_pixels<T: PixelWriter<U>, U: PixelLineWriter>(
    pixels_per_scan_line: UnsignedInt32,
    pixel_format: EfiGraphicsPixelFormat,
    frame_buffer: &mut [UnsignedInt8],
    mut pixels: T,
) -> Result<(), ()> {
    match pixel_format {
        PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR => 'a: loop {
            match pixels.next() {
                Some((mut colors, pos)) => {
                    let mut pixel_start_pos =
                        pixels_per_scan_line as UnsignedIntNative * pos.y() + pos.x();
                    let mut iter = frame_buffer.iter_mut();
                    if pixel_start_pos > 0 {
                        iter.nth(pixel_start_pos * 4 - 1);
                    }
                    'b: loop {
                        match (
                            colors.next(),
                            iter.next(),
                            iter.next(),
                            iter.next(),
                            iter.next(),
                        ) {
                            (
                                Some(color_opt),
                                Some(red_target),
                                Some(green_target),
                                Some(blue_target),
                                Some(_),
                            ) => {
                                match color_opt {
                                    Some(color) => {
                                        *red_target = color.red();
                                        *green_target = color.green();
                                        *blue_target = color.blue();
                                    }
                                    None => (),
                                }
                                pixel_start_pos += 1;
                            }
                            (None, Some(_), Some(_), Some(_), Some(_)) => break 'b (),
                            (Some(_), None, _, _, _) => break 'b (),
                            (None, None, _, _, _) => break 'b (),
                            _ => return Err(()),
                        }
                    }
                }
                None => break 'a Ok(()),
            }
        },
        PIXEL_BLUE_GREEN_RED_RESERVED8_BIT_PER_COLOR => 'a: loop {
            match pixels.next() {
                Some((mut colors, pos)) => {
                    let mut pixel_start_pos =
                        pixels_per_scan_line as UnsignedIntNative * pos.y() + pos.x();
                    let mut iter = frame_buffer.iter_mut();
                    if pixel_start_pos > 0 {
                        iter.nth(pixel_start_pos * 4 - 1);
                    }
                    'b: loop {
                        match (
                            colors.next(),
                            iter.next(),
                            iter.next(),
                            iter.next(),
                            iter.next(),
                        ) {
                            (
                                Some(color_opt),
                                Some(blue_target),
                                Some(green_target),
                                Some(red_target),
                                Some(_),
                            ) => {
                                match color_opt {
                                    Some(color) => {
                                        *red_target = color.red();
                                        *green_target = color.green();
                                        *blue_target = color.blue();
                                    }
                                    None => (),
                                }
                                pixel_start_pos += 1;
                            }
                            (None, Some(_), Some(_), Some(_), Some(_)) => break 'b (),
                            (Some(_), None, _, _, _) => break 'b (),
                            (None, None, _, _, _) => break 'b (),
                            _ => return Err(()),
                        }
                    }
                }
                None => break 'a Ok(()),
            }
        },
        _ => Err(()),
    }
}
