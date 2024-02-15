use core::slice;

use common::uefi::{
    constant::{efi_memory_type::EFI_LOADER_DATA, efi_status::EFI_ABORTED},
    data_type::basic_type::{
        Char16, EfiStatus, UnsignedInt16, UnsignedInt32, UnsignedInt64, UnsignedInt8,
        UnsignedIntNative,
    },
    protocol::efi_simple_text_output_protocol::EfiSimpleTextOutputProtocol,
    table::efi_boot_services::EfiBootServices,
};
use utf16_literal::utf16;

pub struct ConOut<'a> {
    boot_services: &'a EfiBootServices,
    cout: &'a EfiSimpleTextOutputProtocol,
}

pub struct WritedBuffer<'a>(&'a [UnsignedInt8]);
impl WritedBuffer<'_> {
    pub fn get_buffer(&self) -> &[UnsignedInt8] {
        self.0
    }
    pub fn get_buffer_size(&self) -> UnsignedIntNative {
        self.0.len()
    }
}

impl<'a> ConOut<'a> {
    pub fn new(boot_services: &'a EfiBootServices, cout: &'a EfiSimpleTextOutputProtocol) -> Self {
        Self {
            boot_services,
            cout,
        }
    }

    pub fn free_writed_buffer(&self, buf: WritedBuffer<'a>) -> Result<(), EfiStatus> {
        let _ = match self.boot_services.free_pool(buf.0) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };
        Ok(())
    }

    pub fn get_writed_buffer(&self, v: ValueWithFormat) -> Result<WritedBuffer<'a>, EfiStatus> {
        let byte_length = v.get_byte_length();
        let buf = match self
            .boot_services
            .allocate_pool(EFI_LOADER_DATA, byte_length)
        {
            Ok(buf) => buf,
            Err(v) => return Err(v),
        };
        buf.fill(0);
        let _ = match v.write_as_char16(buf) {
            Ok(res) => res,
            Err(_) => {
                let _ = match self
                    .cout
                    .output_string(utf16!("Failed to convert and write value to buffer\r\n\0"))
                {
                    Ok(res) => res,
                    Err(v) => return Err(v),
                };
                return Err(EFI_ABORTED);
            }
        };
        Ok(WritedBuffer(buf))
    }

    pub fn print(&self, v: ValueWithFormat, return_line: bool) -> Result<(), EfiStatus> {
        let buf = match self.get_writed_buffer(v) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };
        if buf.0.len() % 2 != 0 {
            let _ = match self
                .cout
                .output_string(utf16!("Buffer length is not even\r\n\0"))
            {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
            return Err(EFI_ABORTED);
        }
        let _ = match self.cout.output_string(unsafe {
            slice::from_raw_parts(buf.0.as_ptr() as *const Char16, buf.0.len() / 2)
        }) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };
        let _ = match self.free_writed_buffer(buf) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };
        if return_line {
            let _ = match self.cout.output_string(utf16!("\r\n\0")) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub enum UnsignedIntegerDigitCount {
    None,
    Minimum {
        min_count: UnsignedInt8,
        fill: Char16,
    },
    Fixed {
        count: UnsignedInt8,
        fill: Char16,
    },
}

#[derive(Clone, Copy)]
pub enum UnsignedIntegerBase {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}
impl UnsignedIntegerBase {
    pub fn base_value(&self) -> UnsignedInt8 {
        match self {
            UnsignedIntegerBase::Binary => 2,
            UnsignedIntegerBase::Octal => 8,
            UnsignedIntegerBase::Decimal => 10,
            UnsignedIntegerBase::Hexadecimal => 16,
        }
    }

    pub const fn prefix_as_char16(&self) -> &'static [Char16] {
        match self {
            UnsignedIntegerBase::Binary => utf16!("0b"),
            UnsignedIntegerBase::Octal => utf16!("0o"),
            UnsignedIntegerBase::Decimal => utf16!(""),
            UnsignedIntegerBase::Hexadecimal => utf16!("0x"),
        }
    }
}

#[derive(Clone, Copy)]
pub struct UnsignedIntegerFormatter {
    digit_count: UnsignedIntegerDigitCount,
    base: UnsignedIntegerBase,
}
impl UnsignedIntegerFormatter {
    pub fn new(digit_count: UnsignedIntegerDigitCount, base: UnsignedIntegerBase) -> Self {
        Self { digit_count, base }
    }

    pub fn digit_count(&self) -> UnsignedIntegerDigitCount {
        self.digit_count
    }
    pub fn base(&self) -> UnsignedIntegerBase {
        self.base
    }
}

#[derive(Clone, Copy)]
pub enum ValueWithFormat<'a> {
    String(&'a [Char16]),
    UnsignedInt64(UnsignedInt64, UnsignedIntegerFormatter),
    UnsignedInt32(UnsignedInt32, UnsignedIntegerFormatter),
    UnsignedInt16(UnsignedInt16, UnsignedIntegerFormatter),
    UnsignedInt8(UnsignedInt8, UnsignedIntegerFormatter),
    UnsignedIntNative(UnsignedIntNative, UnsignedIntegerFormatter),
}

impl ValueWithFormat<'_> {
    pub fn get_byte_length(&self) -> UnsignedIntNative {
        macro_rules! get_byte_length_unsigned_integer {
            ( $t:ty, $v:ident, $formatter:ident ) => {{
                let base = $formatter.base().base_value() as $t;
                let mut mult = *$v;
                let mut len = 0;
                'b: loop {
                    mult /= base;
                    len += 1;
                    if mult == 0 {
                        break 'b ();
                    }
                }
                let prefix = $formatter.base().prefix_as_char16();
                match $formatter.digit_count() {
                    UnsignedIntegerDigitCount::None => (prefix.len() + len + 1) * 2,
                    UnsignedIntegerDigitCount::Minimum { min_count, fill: _ } => {
                        (prefix.len()
                            + if (min_count as UnsignedIntNative) < len {
                                len
                            } else {
                                min_count as UnsignedIntNative
                            }
                            + 1)
                            * 2
                    }
                    UnsignedIntegerDigitCount::Fixed { count, fill: _ } => {
                        (prefix.len() + count as UnsignedIntNative + 1) * 2
                    }
                }
            }};
        }
        match self {
            ValueWithFormat::String(str) => (str.len() + 1) * 2,
            ValueWithFormat::UnsignedInt64(v, formatter) => {
                get_byte_length_unsigned_integer!(UnsignedInt64, v, formatter)
            }
            ValueWithFormat::UnsignedInt32(v, formatter) => {
                get_byte_length_unsigned_integer!(UnsignedInt32, v, formatter)
            }
            ValueWithFormat::UnsignedInt16(v, formatter) => {
                get_byte_length_unsigned_integer!(UnsignedInt16, v, formatter)
            }
            ValueWithFormat::UnsignedInt8(v, formatter) => {
                get_byte_length_unsigned_integer!(UnsignedInt8, v, formatter)
            }
            ValueWithFormat::UnsignedIntNative(v, formatter) => {
                get_byte_length_unsigned_integer!(UnsignedIntNative, v, formatter)
            }
        }
    }

    pub fn write_as_char16(&self, buf: &mut [UnsignedInt8]) -> Result<(), ()> {
        macro_rules! write_as_char16_unsigned_integer {
            ( $t:ty, $v:ident, $formatter:ident ) => {{
                let base = $formatter.base().base_value() as $t;
                match $formatter.digit_count() {
                    UnsignedIntegerDigitCount::None => {
                        let mut buf_iter = buf.iter_mut().rev();
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        let mut v = *$v;
                        'a: loop {
                            let digit = v % base;
                            let digit = if digit < 10 {
                                '0' as Char16 + digit as Char16
                            } else {
                                'A' as Char16 + digit as Char16 - 10
                            };
                            *(match buf_iter.next() {
                                Some(hi) => hi,
                                None => return Err(()),
                            }) = (digit >> UnsignedInt8::BITS) as UnsignedInt8;
                            *(match buf_iter.next() {
                                Some(lo) => lo,
                                None => return Err(()),
                            }) = digit as UnsignedInt8;
                            v /= base;
                            if v == 0 {
                                break 'a ();
                            }
                        }
                        let mut prefix_iter = $formatter.base().prefix_as_char16().iter().rev();
                        'a: loop {
                            match (buf_iter.next(), buf_iter.next(), prefix_iter.next()) {
                                (Some(hi), Some(lo), Some(v)) => {
                                    *lo = *v as UnsignedInt8;
                                    *hi = (v >> UnsignedInt8::BITS) as UnsignedInt8;
                                }
                                (None, _, None) => break 'a Ok(()),
                                _ => break 'a Err(()),
                            }
                        }
                    }
                    UnsignedIntegerDigitCount::Minimum { min_count, fill } => {
                        let mut buf_iter = buf.iter_mut().rev();
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        let mut v = *$v;
                        let mut count = 0;
                        'a: loop {
                            let digit = v % base;
                            let digit = if digit < 10 {
                                '0' as Char16 + digit as Char16
                            } else {
                                'A' as Char16 + digit as Char16 - 10
                            };
                            *(match buf_iter.next() {
                                Some(hi) => hi,
                                None => return Err(()),
                            }) = (digit >> UnsignedInt8::BITS) as UnsignedInt8;
                            *(match buf_iter.next() {
                                Some(lo) => lo,
                                None => return Err(()),
                            }) = digit as UnsignedInt8;
                            v /= base;
                            count += 1;
                            if v == 0 {
                                break 'a ();
                            }
                        }
                        while count < min_count {
                            match (buf_iter.next(), buf_iter.next()) {
                                (Some(hi), Some(lo)) => {
                                    *lo = fill as UnsignedInt8;
                                    *hi = (fill >> UnsignedInt8::BITS) as UnsignedInt8;
                                }
                                _ => return Err(()),
                            }
                            count += 1;
                        }
                        let mut prefix_iter = $formatter.base().prefix_as_char16().iter().rev();
                        'a: loop {
                            match (buf_iter.next(), buf_iter.next(), prefix_iter.next()) {
                                (Some(hi), Some(lo), Some(v)) => {
                                    *lo = *v as UnsignedInt8;
                                    *hi = (v >> UnsignedInt8::BITS) as UnsignedInt8;
                                }
                                (None, _, None) => break 'a Ok(()),
                                _ => break 'a Err(()),
                            }
                        }
                    }
                    UnsignedIntegerDigitCount::Fixed {
                        count: target_count,
                        fill,
                    } => {
                        let mut buf_iter = buf.iter_mut().rev();
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        match buf_iter.next() {
                            Some(_) => (),
                            None => return Err(()),
                        }
                        let mut v = *$v;
                        let mut count = 0;
                        'a: loop {
                            let digit = v % base;
                            let digit = if digit < 10 {
                                '0' as Char16 + digit as Char16
                            } else {
                                'A' as Char16 + digit as Char16 - 10
                            };
                            *(match buf_iter.next() {
                                Some(hi) => hi,
                                None => return Err(()),
                            }) = (digit >> UnsignedInt8::BITS) as UnsignedInt8;
                            *(match buf_iter.next() {
                                Some(lo) => lo,
                                None => return Err(()),
                            }) = digit as UnsignedInt8;
                            v /= base;
                            count += 1;
                            if v == 0 || count == target_count {
                                break 'a ();
                            }
                        }
                        while count < target_count {
                            match (buf_iter.next(), buf_iter.next()) {
                                (Some(hi), Some(lo)) => {
                                    *lo = fill as UnsignedInt8;
                                    *hi = (fill >> UnsignedInt8::BITS) as UnsignedInt8;
                                }
                                _ => return Err(()),
                            }
                            count += 1;
                        }
                        let mut prefix_iter = $formatter.base().prefix_as_char16().iter().rev();
                        'a: loop {
                            match (buf_iter.next(), buf_iter.next(), prefix_iter.next()) {
                                (Some(hi), Some(lo), Some(v)) => {
                                    *lo = *v as UnsignedInt8;
                                    *hi = (v >> UnsignedInt8::BITS) as UnsignedInt8;
                                }
                                (None, _, None) => break 'a Ok(()),
                                _ => break 'a Err(()),
                            }
                        }
                    }
                }
            }};
        }
        match self {
            ValueWithFormat::String(str) => {
                buf.fill(0);
                let mut buf_iter = buf.iter_mut();
                let mut str_iter = str.iter();
                'a: loop {
                    match (buf_iter.next(), buf_iter.next(), str_iter.next()) {
                        (Some(lo), Some(hi), Some(v)) => {
                            *lo = *v as UnsignedInt8;
                            *hi = (v >> UnsignedInt8::BITS) as UnsignedInt8;
                        }
                        (Some(_), Some(_), None) => break 'a (),
                        _ => return Err(()),
                    }
                }
                match buf_iter.next() {
                    Some(_) => Err(()),
                    None => Ok(()),
                }
            }
            ValueWithFormat::UnsignedInt64(v, formatter) => {
                write_as_char16_unsigned_integer!(UnsignedInt64, v, formatter)
            }
            ValueWithFormat::UnsignedInt32(v, formatter) => {
                write_as_char16_unsigned_integer!(UnsignedInt32, v, formatter)
            }
            ValueWithFormat::UnsignedInt16(v, formatter) => {
                write_as_char16_unsigned_integer!(UnsignedInt16, v, formatter)
            }
            ValueWithFormat::UnsignedInt8(v, formatter) => {
                write_as_char16_unsigned_integer!(UnsignedInt8, v, formatter)
            }
            ValueWithFormat::UnsignedIntNative(v, formatter) => {
                write_as_char16_unsigned_integer!(UnsignedIntNative, v, formatter)
            }
        }
    }
}
