use core::slice::Iter;

use crate::{
    pixel_writer::{pixel_color::PixelColor, PixelLineWriter, PixelWriter},
    util::vector2::Vector2,
};

use super::font::get_font_data;

pub struct FontWriter {
    color: PixelColor,
    pos: Vector2<u32>,
    font_iter: Iter<'static, u8>,
    index: usize,
}

impl FontWriter {
    pub fn new(color: PixelColor, pos: Vector2<u32>, ch: u8) -> Self {
        Self {
            color,
            pos,
            font_iter: get_font_data(ch).iter(),
            index: 0,
        }
    }
}

impl Iterator for FontWriter {
    type Item = (FontWriterLine, Vector2<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.font_iter.next() {
            Some(v) => {
                let ret = (
                    FontWriterLine::new(self.color, *v),
                    Vector2::new(self.pos.x() as usize, self.pos.y() as usize + self.index),
                );
                self.index += 1;
                Some(ret)
            }
            None => None,
        }
    }
}

impl PixelWriter<FontWriterLine> for FontWriter {}

pub struct FontWriterLine {
    color: PixelColor,
    value: u8,
    index: usize,
}

impl FontWriterLine {
    pub const fn new(color: PixelColor, value: u8) -> Self {
        Self {
            color,
            value,
            index: 0,
        }
    }
}

impl Iterator for FontWriterLine {
    type Item = Option<PixelColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < u8::BITS as usize {
            let ret = if ((self.value << self.index) & 0x80) == 0 {
                None
            } else {
                Some(self.color)
            };
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl PixelLineWriter for FontWriterLine {}

pub const FONT_WIDTH: u32 = 8;
pub const FONT_HEIGHT: u32 = 16;
