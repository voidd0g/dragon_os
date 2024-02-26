pub mod unsigned_integer;

pub trait ToIterStr {
    fn to_iter_str(&self, formatter: IterStrFormat) -> impl Iterator<Item = u8>;
}

#[derive(Clone, Copy)]
pub struct IterStrFormat {
    radix: Option<Radix>,
    prefix: Option<bool>,
    padding: Option<Padding>,
}
impl IterStrFormat {
    pub fn get_radix_opt(&self) -> Option<Radix> {
        self.radix
    }

    pub fn get_prefix_opt(&self) -> Option<bool> {
        self.prefix
    }

    pub fn get_padding_opt(&self) -> Option<Padding> {
        self.padding
    }

    pub const fn new(radix: Option<Radix>, prefix: Option<bool>, padding: Option<Padding>) -> Self {
        Self {
            radix,
            prefix,
            padding,
        }
    }

    pub const fn none() -> Self {
        Self {
            radix: None,
            prefix: None,
            padding: None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Padding(u8, usize);
impl Padding {
    pub const fn new(letter: u8, count: usize) -> Self {
        Self(letter, count)
    }

    pub fn get_padding_letter(&self) -> u8 {
        self.0
    }

    pub fn get_padding_length(&self) -> usize {
        self.1
    }
}

#[derive(Clone, Copy)]
pub enum Radix {
    Hexadecimal,
    Decimal,
    Octal,
    Binary,
}

impl Radix {
    pub const fn get_header(&self) -> &'static [u8] {
        match self {
            Radix::Hexadecimal => b"0x",
            Radix::Decimal => b"",
            Radix::Octal => b"0o",
            Radix::Binary => b"0b",
        }
    }

    pub const fn get_value(&self) -> u8 {
        match self {
            Radix::Hexadecimal => 16,
            Radix::Decimal => 10,
            Radix::Octal => 8,
            Radix::Binary => 2,
        }
    }
}

impl ToIterStr for &[u8] {
    fn to_iter_str(&self, _: IterStrFormat) -> impl Iterator<Item = u8> {
        self.iter().map(|v| *v)
    }
}

impl<const N: usize> ToIterStr for [u8; N] {
    fn to_iter_str(&self, _: IterStrFormat) -> impl Iterator<Item = u8> {
        self.iter().map(|v| *v)
    }
}
