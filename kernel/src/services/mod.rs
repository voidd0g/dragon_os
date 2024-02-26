use common::{
    argument::FrameBufferConfig,
    uefi::{
        constant::efi_graphics_pixel_format::{
            PIXEL_BLUE_GREEN_RED_RESERVED8_BIT_PER_COLOR,
            PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR,
        },
        data_type::basic_type::EfiGraphicsPixelFormat,
        table::efi_runtime_services::EfiRuntimeServices,
    },
};

use crate::{
    font::font_writer::{FontWriter, FONT_HEIGHT, FONT_WIDTH},
    pixel_writer::{pixel_color::PixelColor, PixelLineWriter, PixelWriter},
    util::vector2::Vector2,
};

pub struct Services<'a> {
    draw_services: DrawServices<'a>,
    time_services: TimeServices<'a>,
}

impl<'a> Services<'a> {
    pub const fn new(
        frame_buffer_config: &'a FrameBufferConfig,
        runtime_services: &'a EfiRuntimeServices,
    ) -> Self {
        Self {
            draw_services: DrawServices::new(frame_buffer_config),
            time_services: TimeServices::new(runtime_services),
        }
    }

    pub fn draw_services(&self) -> &DrawServices {
        &self.draw_services
    }

    pub fn time_services(&self) -> &TimeServices {
        &self.time_services
    }
}

pub struct TimeServices<'a> {
    runtime_services: &'a EfiRuntimeServices,
}

impl<'a> TimeServices<'a> {
    pub const fn new(runtime_services: &'a EfiRuntimeServices) -> Self {
        Self { runtime_services }
    }

    pub fn wait_for_nano_seconds(&self, nano_seconds: u32) -> Result<(), ()> {
        let wait_for = match self.runtime_services.get_time(None) {
            Ok((cur_time, _)) => cur_time,
            Err(_) => return Err(()),
        }
        .add_nanosecond(nano_seconds);
        'a: loop {
            if wait_for
                > match self.runtime_services.get_time(None) {
                    Ok((cur_time, _)) => cur_time,
                    Err(_) => return Err(()),
                }
            {
                break 'a Ok(());
            }
        }
    }
}

pub struct DrawServices<'a> {
    frame_buffer_config: &'a FrameBufferConfig,
}

impl<'a> DrawServices<'a> {
    pub const fn new(frame_buffer_config: &'a FrameBufferConfig) -> Self {
        Self {
            frame_buffer_config,
        }
    }

    pub fn put_pixel<T: PixelWriter<U>, U: PixelLineWriter>(
        &self,
        color: PixelColor,
        pos: Vector2<usize>,
    ) -> Result<(), ()> {
        put_pixel(
            self.frame_buffer_config.pixels_per_scan_line(),
            self.frame_buffer_config.pixel_format(),
            self.frame_buffer_config.frame_buffer(),
            color,
            pos,
        )
    }

    pub fn put_pixels<T: PixelWriter<U>, U: PixelLineWriter>(&self, pixels: T) -> Result<(), ()> {
        put_pixels(
            self.frame_buffer_config.pixels_per_scan_line(),
            self.frame_buffer_config.pixel_format(),
            self.frame_buffer_config.frame_buffer(),
            pixels,
        )
    }

    pub fn output_string(
        &self,
        elements: &mut [&mut dyn Iterator<Item = u8>],
        color: PixelColor,
        start_pos: Vector2<u32>,
    ) -> Result<(), ()> {
        let mut cur_pos = start_pos;
        let mut elements_iter = elements.iter_mut();
        'a: loop {
            match elements_iter.next() {
                Some(element) => 'b: loop {
                    match element.next() {
                        Some(c) => match c {
                            b'\n' => {
                                cur_pos = Vector2::new(start_pos.x(), cur_pos.y() + FONT_HEIGHT)
                            }
                            c => {
                                let _ = match self.put_pixels(FontWriter::new(color, cur_pos, c)) {
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
}

#[macro_export]
macro_rules! output_string {
	( $services:expr, $color:expr, $start_pos:expr, [ $( $element:expr, )* ] ) => {
		$services.draw_services().output_string(&mut [ $( &mut $element, )* ], $color, $start_pos)
	};
	( $services:expr, $color:expr, $start_pos:expr, [ $( $element:expr ),* ] ) => {
		$services.draw_services().output_string(&mut [ $( &mut $element, )* ], $color, $start_pos)
	};
}

pub fn put_pixel(
    pixels_per_scan_line: u32,
    pixel_format: EfiGraphicsPixelFormat,
    frame_buffer: &mut [u8],
    color: PixelColor,
    pos: Vector2<usize>,
) -> Result<(), ()> {
    match pixel_format {
        PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR => {
            let pixel_start_pos = pixels_per_scan_line as usize * pos.y() + pos.x();
            let mut iter = frame_buffer.iter_mut();
            match (
                iter.nth(pixel_start_pos),
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
            let pixel_start_pos = pixels_per_scan_line as usize * pos.y() + pos.x();
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
    pixels_per_scan_line: u32,
    pixel_format: EfiGraphicsPixelFormat,
    frame_buffer: &mut [u8],
    mut pixels: T,
) -> Result<(), ()> {
    match pixel_format {
        PIXEL_RED_GREEN_BLUE_RESERVED8_BIT_PER_COLOR => 'a: loop {
            match pixels.next() {
                Some((mut colors, pos)) => {
                    let mut pixel_start_pos = pixels_per_scan_line as usize * pos.y() + pos.x();
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
                    let mut pixel_start_pos = pixels_per_scan_line as usize * pos.y() + pos.x();
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
