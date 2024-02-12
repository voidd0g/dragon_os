use crate::uefi::data_types::basic_types::{UINT32, UINT64};

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id4
#[repr(C)]
pub struct EFI_TABLE_HEADER{
    Signature:  UINT64,
    Revision:   UINT32,
    HeaderSize: UINT32,
    CRC32:      UINT32,
    Reserved:   UINT32,
}