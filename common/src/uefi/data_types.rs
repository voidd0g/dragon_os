use core::ffi::c_void;

pub type BOOLEAN = u8;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = i8;
pub type CHAR16 = u16;
pub type UINT64 = u64;
pub type UINT32 = u32;

pub type VOID = c_void;

#[repr(usize)]
pub enum EFI_STATUS {
    Success = 0,
    Aborted = 21,
}

pub type EFI_HANDLE = *mut VOID;

pub type EFI_TPL = UINTN;

#[repr(C)]
pub enum EFI_ALLOCATE_TYPE {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
pub enum EFI_MEMORY_TYPE {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType,
}

pub type EFI_PHYSICAL_ADDRESS = UINT64;
pub type EFI_VIRTUAL_ADDRESS = UINT64;