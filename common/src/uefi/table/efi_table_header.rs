use crate::uefi::data_type::basic_type::{UnsignedInt32, UnsignedInt64};

/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id4
#[repr(C)]
pub struct EfiTableHeader {
    signature: UnsignedInt64,
    revision: UnsignedInt32,
    header_size: UnsignedInt32,
    crc32: UnsignedInt32,
    reserved: UnsignedInt32,
}
