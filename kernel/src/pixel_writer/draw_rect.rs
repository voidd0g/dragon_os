use crate::util::vector2::Vector2;

use super::{pixel_color::PixelColor, PixelLineWriter, PixelWriter};

pub struct DrawRect {
    color: PixelColor,
    pos: Vector2<u32>,
    size: Vector2<u32>,
    index: usize,
}

impl DrawRect {
    pub const fn new(color: PixelColor, pos: Vector2<u32>, size: Vector2<u32>) -> Self {
        Self {
            color,
            pos,
            size,
            index: 0,
        }
    }
}

impl Iterator for DrawRect {
    type Item = (DrawRectLine, Vector2<usize>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size.y() as usize {
            let ret = (
                DrawRectLine::new(self.color, self.size.x()),
                Vector2::new(self.pos.x() as usize, self.pos.y() as usize + self.index),
            );
            self.index += 1;
            Some(ret)
        } else {
            None
        }
    }
}

impl PixelWriter<DrawRectLine> for DrawRect {}

pub struct DrawRectLine {
    color: PixelColor,
    width: u32,
    index: usize,
}

impl DrawRectLine {
    pub const fn new(color: PixelColor, width: u32) -> Self {
        Self {
            color,
            width,
            index: 0,
        }
    }
}

impl Iterator for DrawRectLine {
    type Item = Option<PixelColor>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.width as usize {
            let ret = self.color;
            self.index += 1;
            Some(Some(ret))
        } else {
            None
        }
    }
}

impl PixelLineWriter for DrawRectLine {}
