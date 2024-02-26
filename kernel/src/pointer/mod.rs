use core::slice::Iter;

use crate::{
    pixel_writer::{pixel_color::PixelColor, PixelLineWriter, PixelWriter},
    util::vector2::Vector2,
};

const CURSOR_TEXTURE: [[u8; 4]; 24] = [
    [0b01_00_00_00, 0b00_00_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_01_00_00, 0b00_00_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_01_00, 0b00_00_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_01, 0b00_00_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b01_00_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_01_00_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_01_00, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_01, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b01_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b10_01_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b10_10_01_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b10_10_10_01, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b10_10_10_10, 0b01_00_00_00],
    [0b01_10_10_10, 0b10_10_10_10, 0b10_10_10_10, 0b10_01_00_00],
    [0b01_10_10_10, 0b10_10_10_01, 0b01_01_01_01, 0b01_01_01_00],
    [0b01_10_10_10, 0b10_10_10_01, 0b00_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b10_01_01_10, 0b01_00_00_00, 0b00_00_00_00],
    [0b01_10_10_10, 0b01_00_01_10, 0b01_00_00_00, 0b00_00_00_00],
    [0b01_10_10_01, 0b00_00_00_01, 0b10_01_00_00, 0b00_00_00_00],
    [0b01_10_01_00, 0b00_00_00_01, 0b10_01_00_00, 0b00_00_00_00],
    [0b01_01_00_00, 0b00_00_00_00, 0b01_10_01_00, 0b00_00_00_00],
    [0b01_00_00_00, 0b00_00_00_00, 0b01_10_01_00, 0b00_00_00_00],
    [0b00_00_00_00, 0b00_00_00_00, 0b00_01_01_01, 0b00_00_00_00],
    [0b00_00_00_00, 0b00_00_00_00, 0b00_01_01_01, 0b00_00_00_00],
];

pub struct PointerWriter {
    pos: Vector2<u32>,
    cursor_iter: Iter<'static, [u8; 4]>,
    index: usize,
}

impl PointerWriter {
    pub fn new(pos: Vector2<u32>) -> Self {
        Self {
            pos,
            cursor_iter: CURSOR_TEXTURE.iter(),
            index: 0,
        }
    }
}

impl Iterator for PointerWriter {
    type Item = (PointerWriterLine, Vector2<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor_iter.next() {
            Some(v) => {
                let ret = (
                    PointerWriterLine::new(v),
                    Vector2::new(self.pos.x() as usize, self.pos.y() as usize + self.index),
                );
                self.index += 1;
                Some(ret)
            }
            None => None,
        }
    }
}

impl PixelWriter<PointerWriterLine> for PointerWriter {}

pub struct PointerWriterLine {
    value: &'static [u8; 4],
    index: usize,
}

impl PointerWriterLine {
    pub const fn new(value: &'static [u8; 4]) -> Self {
        Self { value, index: 0 }
    }
}

impl Iterator for PointerWriterLine {
    type Item = Option<PixelColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < u8::BITS as usize * self.value.len() {
            let ret = match (self.value[(self.index / u8::BITS as usize) % self.value.len()]
                << self.index)
                & 0xC0
            {
                0x80 => Some(PixelColor::new(255, 255, 255)),
                0x40 => Some(PixelColor::new(0, 0, 0)),
                _ => None,
            };
            self.index += 2;
            Some(ret)
        } else {
            None
        }
    }
}

impl PixelLineWriter for PointerWriterLine {}
