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

pub type EFI_TPL = UnsignedIntNative;

pub type EFI_EVENT = *const Void;

pub type EFI_PHYSICAL_ADDRESS = UnsignedInt64;
pub type EFI_VIRTUAL_ADDRESS = UnsignedInt64;

pub type EFI_KEY_TOGGLE_STATE = UnsignedInt8;

pub type EFI_ALLOCATE_TYPE = UnsignedInt32;

pub type EFI_MEMORY_TYPE = UnsignedInt32;

pub type EFI_TIMER_DELAY = UnsignedInt32;

pub type EFI_INTERFACE_TYPE = UnsignedInt32;

pub type EFI_LOCATE_SEARCH_TYPE = UnsignedInt32;

pub type C_VARIABLE_ARGUMENT = *const Void;

pub type EFI_GRAPHICS_PIXEL_FORMAT = UnsignedInt32;

pub type EFI_GRAPHICS_OUTPUT_BLT_OPERATION = UnsignedInt32;

#[repr(C)]
pub struct EFI_GUID {
    Data1: UnsignedInt32,
    Data2: UnsignedInt16,
    Data3: UnsignedInt16,
    Data4: [UnsignedInt8; 8],
}

impl EFI_GUID {
    pub const fn new(data1: UnsignedInt32, data2: UnsignedInt16, data3: UnsignedInt16, data4: [UnsignedInt8; 8]) -> Self {
        Self {
            Data1: data1,
            Data2: data2,
            Data3: data3,
            Data4: data4,
        }
    }
}
