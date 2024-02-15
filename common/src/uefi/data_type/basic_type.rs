/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/02_Overview.html#data-types
use core::ffi::c_void;

pub type Boolean = u8;
pub type IntNative = isize;
pub type UnsignedIntNative = usize;
pub type Int8 = i8;
pub type Int16 = i16;
pub type Int32 = i32;
pub type UnsignedInt8 = u8;
pub type Char16 = u16;
pub type UnsignedInt64 = u64;
pub type UnsignedInt32 = u32;
pub type UnsignedInt16 = u16;

pub type Void = c_void;

pub type EfiHandle = *const Void;

pub type EfiStatus = UnsignedIntNative;

pub type EfiTpl = UnsignedIntNative;

pub type EfiEvent = *const Void;

pub type EfiPhysicalAddress = UnsignedInt64;
pub type EfiVirtualAddress = UnsignedInt64;

pub type EfiKeyToggleState = UnsignedInt8;

pub type EfiAllocateType = UnsignedInt32;

pub type EfiMemoryType = UnsignedInt32;

pub type EfiTimerDelay = UnsignedInt32;

pub type EfiInterfaceType = UnsignedInt32;

pub type EfiLocateSearchType = UnsignedInt32;

pub type CVariableLengthArgument = *const Void;

pub type EfiGraphicsPixelFormat = UnsignedInt32;

pub type EfiGraphicsOutputBltOperation = UnsignedInt32;

#[repr(C)]
pub struct EfiGuid {
    data1: UnsignedInt32,
    data2: UnsignedInt16,
    data3: UnsignedInt16,
    data4: [UnsignedInt8; 8],
}

impl EfiGuid {
    pub const fn new(
        data1: UnsignedInt32,
        data2: UnsignedInt16,
        data3: UnsignedInt16,
        data4: [UnsignedInt8; 8],
    ) -> Self {
        Self {
            data1,
            data2,
            data3,
            data4,
        }
    }
}
