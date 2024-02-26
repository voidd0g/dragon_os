use crate::uefi::data_type::{
    basic_type::{Boolean, EfiGuid, EfiPhysicalAddress, EfiResetType, EfiStatus, Void},
    efi_capsule_header::EfiCapsuleHeader,
    efi_memory_descriptor::EfiMemoryDescriptor,
    efi_time::EfiTime,
    efi_time_capabilities::EfiTimeCapabilities,
};

use super::efi_table_header::EfiTableHeader;

type EfiGetTime = unsafe extern "efiapi" fn(
    time_out: *mut EfiTime,
    capabilities_out_optional: *mut EfiTimeCapabilities,
) -> EfiStatus;
type EfiSetTime = unsafe extern "efiapi" fn(time: *const EfiTime) -> EfiStatus;
type EfiGetWakeupTime = unsafe extern "efiapi" fn(
    enable_out: *mut Boolean,
    pending_out: *mut Boolean,
    time_out: *mut EfiTime,
) -> EfiStatus;
type EfiSetWakeupTime =
    unsafe extern "efiapi" fn(enable: Boolean, time_optional: *const EfiTime) -> EfiStatus;
type EfiSetVirtualAddressMap = unsafe extern "efiapi" fn(
    memory_map_size: usize,
    descriptor_size: usize,
    descriptor_version: u32,
    virtual_map: *const EfiMemoryDescriptor,
) -> EfiStatus;
type EfiConvertPointer =
    unsafe extern "efiapi" fn(debug_disposition: usize, address: *const *const Void) -> EfiStatus;
type EfiGetVariable = unsafe extern "efiapi" fn(
    variable_name: *const u16,
    vendor_guid: *const EfiGuid,
    attributes_out_optional: *mut u32,
    data_size_in_out: *mut usize,
    data_out_optional: *mut Void,
) -> EfiStatus;
type EfiGetNextVariableName = unsafe extern "efiapi" fn(
    variable_name_size_in_out: *mut usize,
    variable_name_in_out: *mut usize,
    vendor_guid_in_out: *mut EfiGuid,
) -> EfiStatus;
type EfiSetVariable = unsafe extern "efiapi" fn(
    variable_name: *const u16,
    vendor_guid: *const EfiGuid,
    attributes: u32,
    data_size: usize,
    data: *const Void,
) -> EfiStatus;
type EfiGetNextHighMonoCount = unsafe extern "efiapi" fn(high_count_out: *mut u32) -> EfiStatus;
type EfiResetSystem = unsafe extern "efiapi" fn(
    reset_type: EfiResetType,
    reset_status: EfiStatus,
    data_size: usize,
    reset_data_optional: *const Void,
) -> EfiStatus;
type EfiUpdateCapsule = unsafe extern "efiapi" fn(
    capsule_header_array: *const *const EfiCapsuleHeader,
    capsule_count: usize,
    scatter_gather_list_optional: EfiPhysicalAddress,
) -> EfiStatus;
type EfiQueryCapsuleCapabilities = unsafe extern "efiapi" fn(
    capsule_header_array: *const *const EfiCapsuleHeader,
    capsule_count: usize,
    maximum_capsule_size_out: *mut u64,
    reset_type_out: *mut EfiResetType,
) -> EfiStatus;
type EfiQueryVariableInfo = unsafe extern "efiapi" fn(
    attributes: u32,
    maximum_variable_storage_size_out: *mut u64,
    remaining_variable_storage_size: *mut u64,
    maximum_variable_size: *mut u64,
) -> EfiStatus;

#[repr(C)]
pub struct EfiRuntimeServices {
    hdr: EfiTableHeader,

    get_time: EfiGetTime,
    set_time: EfiSetTime,
    get_wakeup_time: EfiGetWakeupTime,
    set_wakeup_time: EfiSetWakeupTime,

    set_virtual_address_map: EfiSetVirtualAddressMap,
    convert_pointer: EfiConvertPointer,

    get_variable: EfiGetVariable,
    get_next_variable_name: EfiGetNextVariableName,
    set_variable: EfiSetVariable,

    get_next_high_monotonic_count: EfiGetNextHighMonoCount,
    reset_system: EfiResetSystem,

    update_capsule: EfiUpdateCapsule,
    query_capsule_capabilities: EfiQueryCapsuleCapabilities,

    query_variable_info: EfiQueryVariableInfo,
}
