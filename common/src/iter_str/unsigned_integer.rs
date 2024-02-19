use core::slice::Iter;

use crate::uefi::data_type::basic_type::{
    UnsignedInt16, UnsignedInt32, UnsignedInt64, UnsignedInt8, UnsignedIntNative,
};

use super::{IterStrFormat, Padding, Radix, ToIterStr};

macro_rules! iter_str_of_unsigned_integer {
    ( $t:ty, $IterStrOf:ident ) => {
        impl ToIterStr for $t {
            fn to_iter_str(&self, formatter: IterStrFormat) -> impl Iterator<Item = UnsignedInt8> {
                $IterStrOf::new(*self, formatter)
            }
        }

        struct $IterStrOf {
            value: $t,
            header_iter_opt: Option<Iter<'static, UnsignedInt8>>,
            padding_letter: UnsignedInt8,
            padding_rest: UnsignedIntNative,
            base: $t,
            cur_div: $t,
        }

        impl $IterStrOf {
            pub fn new(value: $t, formatter: IterStrFormat) -> Self {
                let radix = match formatter.get_radix_opt() {
                    Some(radix) => radix,
                    None => Radix::Decimal,
                };
                let base = radix.get_value() as $t;
                let header_iter_opt = if match formatter.get_prefix_opt() {
                    Some(prefix) => prefix,
                    None => false,
                } {
                    Some(radix.get_header().iter())
                } else {
                    None
                };
                let (padding_letter, padding_count) = match formatter.get_padding_opt() {
                    Some(Padding(padding_letter, padding_count)) => (padding_letter, padding_count),
                    None => (0, 0),
                };
                let mut mult = value;
                let mut len = 0;
                let mut div = 1;
                'b: loop {
                    mult /= base;
                    len += 1;
                    if mult == 0 {
                        break 'b ();
                    }
                    div *= base;
                }
                Self {
                    value,
                    header_iter_opt,
                    padding_letter,
                    padding_rest: if padding_count > len {
                        padding_count - len
                    } else {
                        0
                    },
                    cur_div: div,
                    base,
                }
            }
        }

        impl Iterator for $IterStrOf {
            type Item = UnsignedInt8;

            fn next(&mut self) -> Option<Self::Item> {
                match &mut self.header_iter_opt {
                    Some(header_iter) => match header_iter.next() {
                        Some(header) => return Some(*header),
                        None => self.header_iter_opt = None,
                    },
                    None => (),
                }
                if self.padding_rest > 0 {
                    self.padding_rest -= 1;
                    return Some(self.padding_letter);
                }
                if self.cur_div == 0 {
                    None
                } else {
                    let mut ret = ((self.value / self.cur_div) % self.base) as UnsignedInt8;
                    if ret < 10 {
                        ret = b'0' + ret;
                    } else {
                        ret = b'A' + ret - 10;
                    }
                    self.cur_div /= self.base;
                    Some(ret)
                }
            }
        }
    };
}

iter_str_of_unsigned_integer! {UnsignedInt8, IterStrOfUnsignedInt8}
iter_str_of_unsigned_integer! {UnsignedInt16, IterStrOfUnsignedInt16}
iter_str_of_unsigned_integer! {UnsignedInt32, IterStrOfUnsignedInt32}
iter_str_of_unsigned_integer! {UnsignedInt64, IterStrOfUnsignedInt64}
iter_str_of_unsigned_integer! {UnsignedIntNative, IterStrOfUnsignedIntNative}
