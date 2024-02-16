#[derive(Clone, Copy)]
pub struct Vector2<T: Clone + Copy>(T, T);

impl<T: Clone + Copy> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> T {
        self.0
    }

    pub fn y(&self) -> T {
        self.1
    }
}
