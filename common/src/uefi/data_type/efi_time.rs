use super::basic_type::{Int16, UnsignedInt16, UnsignedInt32, UnsignedInt8};

#[repr(C)]
pub struct EfiTime {
    year: UnsignedInt16,
    month: UnsignedInt8,
    day: UnsignedInt8,
    hour: UnsignedInt8,
    minute: UnsignedInt8,
    second: UnsignedInt8,
    pad1: UnsignedInt8,
    nanosecond: UnsignedInt32,
    time_zone: Int16,
    daylight: UnsignedInt8,
    pad2: UnsignedInt8,
}
