/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types

use core::ffi::c_void;

pub type BOOLEAN = u8;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = u8;
pub type CHAR16 = u16;
pub type UINT64 = u64;
pub type UINT32 = u32;
pub type UINT16 = u16;

pub type VOID = c_void;

pub type EFI_HANDLE = *const VOID;

pub type EFI_STATUS = UINTN;

pub type EFI_TPL = UINTN;

pub type EFI_EVENT = *const VOID;

pub type EFI_PHYSICAL_ADDRESS = UINT64;
pub type EFI_VIRTUAL_ADDRESS = UINT64;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/Apx_D_Status_Codes.html#status-codes
pub const EFI_SUCCESS: EFI_STATUS = 0;
pub const EFI_ABORTED: EFI_STATUS = 21;

#[repr(C)]
pub struct EFI_GUID {
    Data1: UINT32,
    Data2: UINT16,
    Data3: UINT16,
    Data4: [UINT8; 8],
}

impl EFI_GUID {
    pub const fn new(data1: UINT32, data2: UINT16, data3: UINT16, data4: [UINT8; 8]) -> Self {
        Self { Data1: data1, Data2: data2, Data3: data3, Data4: data4 }
    }
}

pub type C_VARIABLE_ARGUMENT = *const VOID;