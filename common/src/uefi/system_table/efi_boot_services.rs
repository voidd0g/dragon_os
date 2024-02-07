use crate::uefi::function_types::{EFI_RAISE_TPL, EFI_RESTORE_TPL};

use super::efi_table_header::EFI_TABLE_HEADER;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html?#efi-boot-services
#[repr(C)]
pub struct EFI_BOOT_SERVICES { 
	Hdr: EFI_TABLE_HEADER,
	RaiseTPL: EFI_RAISE_TPL,
	RestoreTPL: EFI_RESTORE_TPL,
}