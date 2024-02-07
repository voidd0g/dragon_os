use crate::uefi::function_types::{EFI_RAISE_TPL, EFI_RESTORE_TPL};

use super::efi_table_header::EFI_TABLE_HEADER;

#[repr(C)]
pub struct EFI_BOOT_SERVICES { 
	Hdr: EFI_TABLE_HEADER,
	RaiseTPL: EFI_RAISE_TPL,
	RestoreTPL: EFI_RESTORE_TPL,
}