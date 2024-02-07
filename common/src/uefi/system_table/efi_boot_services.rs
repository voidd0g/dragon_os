use crate::uefi::data_types::{common_types::{EFI_STATUS, UINT32, UINTN, VOID}, other_types::{EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, EFI_PHYSICAL_ADDRESS, EFI_TPL}};

use super::{efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR, efi_table_header::EFI_TABLE_HEADER};


type EFI_RAISE_TPL = extern "C" fn (NewTpl: EFI_TPL) -> EFI_TPL;
type EFI_RESTORE_TPL = extern "C" fn (OldTpl: EFI_TPL) -> VOID;
type EFI_ALLOCATE_PAGES = extern "C" fn (Type: EFI_ALLOCATE_TYPE, MemoryType: EFI_MEMORY_TYPE, Pages: UINTN, Memory: *mut EFI_PHYSICAL_ADDRESS) -> EFI_STATUS;
type EFI_FREE_PAGES = extern "C" fn (Memory: EFI_PHYSICAL_ADDRESS, Pages: UINTN) -> EFI_STATUS;
type EFI_GET_MEMORY_MAP = extern "C" fn (MemoryMapSize: *mut UINTN, MemoryMap: *mut EFI_MEMORY_DESCRIPTOR, MapKey: *mut UINTN, DescriptorSize: *mut UINTN, DescriptorVersion: *mut UINT32) -> EFI_STATUS;
type EFI_ALLOCATE_POOL = extern "C" fn (PoolType: EFI_MEMORY_TYPE, Size: UINTN, Buffer: *mut *mut VOID) -> EFI_STATUS;

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-boot-services
#[repr(C)]
pub struct EFI_BOOT_SERVICES { 
	Hdr: EFI_TABLE_HEADER,

	RaiseTPL: EFI_RAISE_TPL,
	RestoreTPL: EFI_RESTORE_TPL,

	AllocatePages: EFI_ALLOCATE_PAGES,
	FreePages: EFI_FREE_PAGES,
	GetMemoryMap: EFI_GET_MEMORY_MAP,
	AllocatePool: EFI_ALLOCATE_POOL,
}