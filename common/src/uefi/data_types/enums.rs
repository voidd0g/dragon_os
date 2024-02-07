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

#[repr(C)]
pub enum EFI_TIMER_DELAY {
    TimerCancel,
    TimerPeriodic,
    TimerRelative,
}

#[repr(C)]
pub enum EFI_INTERFACE_TYPE {
    EFI_NATIVE_INTERFACE,
}

#[repr(C)]
pub enum EFI_LOCATE_SEARCH_TYPE {
    AllHandles,
    ByRegisterNotify,
    ByProtocol,
}