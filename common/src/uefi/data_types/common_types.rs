/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types

use core::ffi::c_void;

pub type BOOLEAN = u8;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = i8;
pub type CHAR16 = u16;
pub type UINT64 = u64;
pub type UINT32 = u32;

pub type VOID = c_void;

pub type EFI_HANDLE = *mut VOID;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html#status-codes
#[repr(usize)]
pub enum EFI_STATUS {
    Success = 0,
    Aborted = 21,
}