use super::{data_types::{BOOLEAN, CHAR16, EFI_ALLOCATE_TYPE, EFI_MEMORY_TYPE, EFI_PHYSICAL_ADDRESS, EFI_STATUS, EFI_TPL, UINT32, UINTN, VOID}, system_table::{efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR, efi_simple_text_output_protocol::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL}};

pub type EFI_TEXT_RESET = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: BOOLEAN) -> EFI_STATUS;
pub type EFI_TEXT_STRING = extern "C" fn (This: *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS;
pub type EFI_RAISE_TPL = extern "C" fn (NewTpl: EFI_TPL) -> EFI_TPL;
pub type EFI_RESTORE_TPL = extern "C" fn (OldTpl: EFI_TPL) -> VOID;
pub type EFI_ALLOCATE_PAGES = extern "C" fn (Type: EFI_ALLOCATE_TYPE, MemoryType: EFI_MEMORY_TYPE, Pages: UINTN, Memory: *mut EFI_PHYSICAL_ADDRESS) -> EFI_STATUS;
pub type EFI_FREE_PAGES = extern "C" fn (Memory: EFI_PHYSICAL_ADDRESS, Pages: UINTN) -> EFI_STATUS;
pub type EFI_GET_MEMORY_MAP = extern "C" fn (MemoryMapSize: *mut UINTN, MemoryMap: *mut EFI_MEMORY_DESCRIPTOR, MapKey: *mut UINTN, DescriptorSize: *mut UINTN, DescriptorVersion: *mut UINT32) -> EFI_STATUS;
pub type EFI_ALLOCATE_POOL = extern "C" fn (PoolType: EFI_MEMORY_TYPE, Size: UINTN, Buffer: *mut *mut VOID) -> EFI_STATUS;