use crate::uefi::data_type::basic_type::{Int16, UnsignedInt16, UnsignedInt32, UnsignedInt8};

#[repr(C)]
pub struct EFI_TIME {
    Year: UnsignedInt16,
    Month: UnsignedInt8,
    Day: UnsignedInt8,
    Hour: UnsignedInt8,
    Minute: UnsignedInt8,
    Second: UnsignedInt8,
    Pad1: UnsignedInt8,
    Nanosecond: UnsignedInt32,
    TimeZone: Int16,
    Daylight: UnsignedInt8,
    Pad2: UnsignedInt8,
}
