use crate::uefi::data_type::{
    basic_type::{
        Boolean, Char16, EfiGuid, EfiPhysicalAddress, EfiResetType, EfiStatus, UnsignedInt32,
        UnsignedInt64, UnsignedIntNative, Void,
    },
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
    memory_map_size: UnsignedIntNative,
    descriptor_size: UnsignedIntNative,
    descriptor_version: UnsignedInt32,
    virtual_map: *const EfiMemoryDescriptor,
) -> EfiStatus;
type EfiConvertPointer = unsafe extern "efiapi" fn(
    debug_disposition: UnsignedIntNative,
    address: *const *const Void,
) -> EfiStatus;
type EfiGetVariable = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    attributes_out_optional: *mut UnsignedInt32,
    data_size_in_out: *mut UnsignedIntNative,
    data_out_optional: *mut Void,
) -> EfiStatus;
type EfiGetNextVariableName = unsafe extern "efiapi" fn(
    variable_name_size_in_out: *mut UnsignedIntNative,
    variable_name_in_out: *mut UnsignedIntNative,
    vendor_guid_in_out: *mut EfiGuid,
) -> EfiStatus;
type EfiSetVariable = unsafe extern "efiapi" fn(
    variable_name: *const Char16,
    vendor_guid: *const EfiGuid,
    attributes: UnsignedInt32,
    data_size: UnsignedIntNative,
    data: *const Void,
) -> EfiStatus;
type EfiGetNextHighMonoCount =
    unsafe extern "efiapi" fn(high_count_out: *mut UnsignedInt32) -> EfiStatus;
type EfiResetSystem = unsafe extern "efiapi" fn(
    reset_type: EfiResetType,
    reset_status: EfiStatus,
    data_size: UnsignedIntNative,
    reset_data_optional: *const Void,
) -> EfiStatus;
type EfiUpdateCapsule = unsafe extern "efiapi" fn(
    capsule_header_array: *const *const EfiCapsuleHeader,
    capsule_count: UnsignedIntNative,
    scatter_gather_list_optional: EfiPhysicalAddress,
) -> EfiStatus;
type EfiQueryCapsuleCapabilities = unsafe extern "efiapi" fn(
    capsule_header_array: *const *const EfiCapsuleHeader,
    capsule_count: UnsignedIntNative,
    maximum_capsule_size_out: *mut UnsignedInt64,
    reset_type_out: *mut EfiResetType,
) -> EfiStatus;
type EfiQueryVariableInfo = unsafe extern "efiapi" fn(
    attributes: UnsignedInt32,
    maximum_variable_storage_size_out: *mut UnsignedInt64,
    remaining_variable_storage_size: *mut UnsignedInt64,
    maximum_variable_size: *mut UnsignedInt64,
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
