use crate::uefi::data_types::{UINT32, UINT64};

#[repr(C)]
pub struct EFI_TABLE_HEADER{
    Signature:  UINT64,
    Revision:   UINT32,
    HeaderSize: UINT32,
    CRC32:      UINT32,
    Reserved:   UINT32,
}