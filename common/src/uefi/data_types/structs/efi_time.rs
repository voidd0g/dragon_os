use crate::uefi::data_types::basic_types::{INT16, UINT16, UINT32, UINT8};

#[repr(C)]
pub struct EFI_TIME {
    Year: UINT16,
    Month: UINT8,
    Day: UINT8,
    Hour: UINT8,
    Minute: UINT8,
    Second: UINT8,
    Pad1: UINT8,
    Nanosecond: UINT32,
    TimeZone: INT16,
    Daylight: UINT8,
    Pad2: UINT8,
}
