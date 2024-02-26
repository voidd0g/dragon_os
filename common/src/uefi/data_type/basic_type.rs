/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types
use core::ffi::c_void;

pub type Boolean = u8;

pub type Void = c_void;

pub type EfiHandle = *const Void;

pub type EfiStatus = usize;

pub type EfiTpl = usize;

pub type EfiEvent = *const Void;

pub type EfiPhysicalAddress = u64;
pub type EfiVirtualAddress = u64;

pub type EfiKeyToggleState = u8;

pub type EfiAllocateType = u32;

pub type EfiMemoryType = u32;

pub type EfiTimerDelay = u32;

pub type EfiInterfaceType = u32;

pub type EfiLocateSearchType = u32;

pub type CVariableLengthArgument = *const Void;

pub type EfiGraphicsPixelFormat = u32;

pub type EfiGraphicsOutputBltOperation = u32;

pub type EfiResetType = u32;

#[repr(C)]
pub struct EfiGuid {
    data1: u32,
    data2: u16,
    data3: u16,
    data4: [u8; 8],
}

impl EfiGuid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}
