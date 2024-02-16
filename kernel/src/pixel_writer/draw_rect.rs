use common::uefi::data_type::basic_type::{UnsignedInt32, UnsignedIntNative};

use crate::util::vector2::Vector2;

use super::{pixel_color::PixelColor, PixelLineWriter, PixelWriter};

pub struct DrawRect {
    color: PixelColor,
    pos: Vector2<UnsignedInt32>,
    size: Vector2<UnsignedInt32>,
    index: UnsignedIntNative,
}

impl DrawRect {
    pub const fn new(
        color: PixelColor,
        pos: Vector2<UnsignedInt32>,
        size: Vector2<UnsignedInt32>,
    ) -> Self {
        Self {
            color,
            pos,
            size,
            index: 0,
        }
    }
}

impl Iterator for DrawRect {
    type Item = (DrawRectLine, Vector2<UnsignedIntNative>);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.size.y() as UnsignedIntNative {
            let ret = (
                DrawRectLine::new(self.color, self.size.x()),
                Vector2::new(
                    self.pos.x() as UnsignedIntNative,
                    self.pos.y() as UnsignedIntNative + self.index,
                ),
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
    width: UnsignedInt32,
    index: UnsignedIntNative,
}

impl DrawRectLine {
    pub const fn new(color: PixelColor, width: UnsignedInt32) -> Self {
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
        if self.index < self.width as UnsignedIntNative {
            let ret = self.color;
            self.index += 1;
            Some(Some(ret))
        } else {
            None
        }
    }
}

impl PixelLineWriter for DrawRectLine {}
