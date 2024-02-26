/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id4
#[repr(C)]
pub struct EfiTableHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}
