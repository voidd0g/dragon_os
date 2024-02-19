pub mod unsigned_integer;

use crate::uefi::data_type::basic_type::{UnsignedInt8, UnsignedIntNative};

pub trait ToIterStr {
    fn to_iter_str(&self, formatter: IterStrFormat) -> impl Iterator<Item = UnsignedInt8>;
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
pub struct Padding(UnsignedInt8, UnsignedIntNative);
impl Padding {
    pub const fn new(letter: UnsignedInt8, count: UnsignedIntNative) -> Self {
        Self(letter, count)
    }

    pub fn get_padding_letter(&self) -> UnsignedInt8 {
        self.0
    }

    pub fn get_padding_length(&self) -> UnsignedIntNative {
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
    pub const fn get_header(&self) -> &'static [UnsignedInt8] {
        match self {
            Radix::Hexadecimal => b"0x",
            Radix::Decimal => b"",
            Radix::Octal => b"0o",
            Radix::Binary => b"0b",
        }
    }

    pub const fn get_value(&self) -> UnsignedInt8 {
        match self {
            Radix::Hexadecimal => 16,
            Radix::Decimal => 10,
            Radix::Octal => 8,
            Radix::Binary => 2,
        }
    }
}

impl ToIterStr for &[UnsignedInt8] {
    fn to_iter_str(&self, _: IterStrFormat) -> impl Iterator<Item = UnsignedInt8> {
        self.iter().map(|v| *v)
    }
}

impl<const N: usize> ToIterStr for [UnsignedInt8; N] {
    fn to_iter_str(&self, _: IterStrFormat) -> impl Iterator<Item = UnsignedInt8> {
        self.iter().map(|v| *v)
    }
}
