pub mod draw_rect;
pub mod pixel_color;

use pixel_color::PixelColor;

use crate::util::vector2::Vector2;

pub trait PixelWriter<T: PixelLineWriter>: Iterator<Item = (T, Vector2<usize>)> {}
pub trait PixelLineWriter: Iterator<Item = Option<PixelColor>> {}
