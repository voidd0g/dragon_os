use common::uefi::data_type::basic_type::UnsignedInt8;

#[derive(Clone, Copy)]
pub struct PixelColor {
    red: UnsignedInt8,
    green: UnsignedInt8,
    blue: UnsignedInt8,
}

impl PixelColor {
    pub const fn new(red: UnsignedInt8, green: UnsignedInt8, blue: UnsignedInt8) -> Self {
        Self { red, green, blue }
    }

    pub fn red(&self) -> UnsignedInt8 {
        self.red
    }

    pub fn green(&self) -> UnsignedInt8 {
        self.green
    }

    pub fn blue(&self) -> UnsignedInt8 {
        self.blue
    }
}
