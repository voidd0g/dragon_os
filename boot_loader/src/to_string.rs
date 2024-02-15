use core::slice;

use common::uefi::{
    constant::efi_memory_type::EFI_LOADER_DATA,
    data_type::basic_type::{Char16, EfiStatus},
    table::efi_boot_services::EFI_BOOT_SERVICES,
};

pub trait ToString {
    fn to_string<'a>(
        &self,
        boot_services: &'a EFI_BOOT_SERVICES,
    ) -> Result<&'a [Char16], EfiStatus>;
}

impl ToString for (u64, u8) {
    fn to_string<'a>(
        &self,
        boot_services: &'a EFI_BOOT_SERVICES,
    ) -> Result<&'a [Char16], EfiStatus> {
        let (efi_status, base) = self;
        let (efi_status, base) = (*efi_status, *base);
        let mut mult = efi_status;
        let mut len = 1;
        'b: loop {
            mult /= base as u64;
            len += 1;
            if mult == 0 {
                break 'b ();
            }
        }
        let buf = match boot_services.allocate_pool(EFI_LOADER_DATA, len * 2) {
            Ok(buf) => buf,
            Err(v) => return Err(v),
        };
        let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Char16, len) };
        let mut iter = buf.iter_mut().rev();
        *iter.next().unwrap() = 0;
        let mut num = efi_status;
        for _ in 0..len - 1 {
            let digit = num as u8 % base as u8;
            if digit < 10 {
                *iter.next().unwrap() = ('0' as u8 + digit) as Char16;
            } else {
                *iter.next().unwrap() = ('A' as u8 + digit - 10) as Char16;
            }
            num /= base as u64;
        }
        Ok(buf)
    }
}

impl ToString for (usize, u8) {
    fn to_string<'a>(
        &self,
        boot_services: &'a EFI_BOOT_SERVICES,
    ) -> Result<&'a [Char16], EfiStatus> {
        let (efi_status, base) = self;
        let (efi_status, base) = (*efi_status, *base);
        let mut mult = efi_status;
        let mut len = 1;
        'b: loop {
            mult /= base as usize;
            len += 1;
            if mult == 0 {
                break 'b ();
            }
        }
        let buf = match boot_services.allocate_pool(EFI_LOADER_DATA, len * 2) {
            Ok(buf) => buf,
            Err(v) => return Err(v),
        };
        let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Char16, len) };
        let mut iter = buf.iter_mut().rev();
        *iter.next().unwrap() = 0;
        let mut num = efi_status;
        for _ in 0..len - 1 {
            let digit = num as u8 % base as u8;
            if digit < 10 {
                *iter.next().unwrap() = ('0' as u8 + digit) as Char16;
            } else {
                *iter.next().unwrap() = ('A' as u8 + digit - 10) as Char16;
            }
            num /= base as usize;
        }
        Ok(buf)
    }
}

impl ToString for (u32, u8) {
    fn to_string<'a>(
        &self,
        boot_services: &'a EFI_BOOT_SERVICES,
    ) -> Result<&'a [Char16], EfiStatus> {
        let (efi_status, base) = self;
        let (efi_status, base) = (*efi_status, *base);
        let mut mult = efi_status;
        let mut len = 1;
        'b: loop {
            mult /= base as u32;
            len += 1;
            if mult == 0 {
                break 'b ();
            }
        }
        let buf = match boot_services.allocate_pool(EFI_LOADER_DATA, len * 2) {
            Ok(buf) => buf,
            Err(v) => return Err(v),
        };
        let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Char16, len) };
        let mut iter = buf.iter_mut().rev();
        *iter.next().unwrap() = 0;
        let mut num = efi_status;
        for _ in 0..len - 1 {
            let digit = num as u8 % base as u8;
            if digit < 10 {
                *iter.next().unwrap() = ('0' as u8 + digit) as Char16;
            } else {
                *iter.next().unwrap() = ('A' as u8 + digit - 10) as Char16;
            }
            num /= base as u32;
        }
        Ok(buf)
    }
}

impl ToString for (i32, u8) {
    fn to_string<'a>(
        &self,
        boot_services: &'a EFI_BOOT_SERVICES,
    ) -> Result<&'a [Char16], EfiStatus> {
        let (efi_status, base) = self;
        let (efi_status, base) = (*efi_status, *base);
        let mut mult = efi_status;
        let mut len = 1;
        'b: loop {
            mult /= base as i32;
            len += 1;
            if mult == 0 {
                break 'b ();
            }
        }
        let buf = match boot_services.allocate_pool(EFI_LOADER_DATA, len * 2) {
            Ok(buf) => buf,
            Err(v) => return Err(v),
        };
        let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Char16, len) };
        let mut iter = buf.iter_mut().rev();
        *iter.next().unwrap() = 0;
        let mut num = efi_status;
        for _ in 0..len - 1 {
            let digit = num as u8 % base as u8;
            if digit < 10 {
                *iter.next().unwrap() = ('0' as u8 + digit) as Char16;
            } else {
                *iter.next().unwrap() = ('A' as u8 + digit - 10) as Char16;
            }
            num /= base as i32;
        }
        Ok(buf)
    }
}
