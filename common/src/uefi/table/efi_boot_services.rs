use core::{
    ptr::{null, null_mut},
    slice,
};

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::{
        basic_type::{
            Boolean, CVariableLengthArgument, Char16, EfiAllocateType, EfiEvent, EfiGuid,
            EfiHandle, EfiInterfaceType, EfiLocateSearchType, EfiMemoryType, EfiPhysicalAddress,
            EfiStatus, EfiTimerDelay, EfiTpl, UnsignedInt32, UnsignedInt64, UnsignedInt8,
            UnsignedIntNative, Void,
        },
        {
            efi_memory_descriptor::EfiMemoryDescriptor,
            efi_open_protocol_information_entry::EfiOpenProtocolInformationEntry,
        },
    },
    protocol::efi_device_path_protocol::EfiDevicePathProtocol,
};

use super::efi_table_header::EfiTableHeader;

type EfiRaiseTpl = unsafe extern "efiapi" fn(new_tpl: EfiTpl) -> EfiTpl;
type EfiRestoreTpl = unsafe extern "efiapi" fn(old_tpl: EfiTpl) -> Void;

type EfiAllocatePages = unsafe extern "efiapi" fn(
    r#type: EfiAllocateType,
    memory_type: EfiMemoryType,
    pages: UnsignedIntNative,
    memory_in_out: *mut EfiPhysicalAddress,
) -> EfiStatus;
type EfiFreePages =
    unsafe extern "efiapi" fn(memory: EfiPhysicalAddress, pages: UnsignedIntNative) -> EfiStatus;
type EfiGetMemoryMap = unsafe extern "efiapi" fn(
    memory_map_size_in_out: *mut UnsignedIntNative,
    memory_map_out: *mut EfiMemoryDescriptor,
    map_key_out: *mut UnsignedIntNative,
    descriptor_size_out: *mut UnsignedIntNative,
    descriptor_version_out: *mut UnsignedInt32,
) -> EfiStatus;
type EfiAllocatePool = unsafe extern "efiapi" fn(
    pool_type: EfiMemoryType,
    size: UnsignedIntNative,
    buffer_out: *mut *const Void,
) -> EfiStatus;
type EfiFreePool = unsafe extern "efiapi" fn(buffer: *const Void) -> EfiStatus;

type EfiCreateEvent = unsafe extern "efiapi" fn(
    r#type: UnsignedInt32,
    notify_tpl: EfiTpl,
    notify_function_optional: Option<EfiEventNotify>,
    notify_context_optional: *const Void,
    event_out: *mut EfiEvent,
) -> EfiStatus;
type EfiEventNotify = unsafe extern "efiapi" fn(event: EfiEvent, context: *const Void) -> Void;
type EfiSetTimer = unsafe extern "efiapi" fn(
    event: EfiEvent,
    r#type: EfiTimerDelay,
    trigger_time: UnsignedInt64,
) -> EfiStatus;
type EfiWaitForEvent = unsafe extern "efiapi" fn(
    number_of_events: UnsignedIntNative,
    event: *const EfiEvent,
    index_out: *mut UnsignedIntNative,
) -> EfiStatus;
type EfiSignalEvent = unsafe extern "efiapi" fn(event: EfiEvent) -> EfiStatus;
type EfiCloseEvent = unsafe extern "efiapi" fn(event: EfiEvent) -> EfiStatus;
type EfiCheckEvent = unsafe extern "efiapi" fn(event: EfiEvent) -> EfiStatus;

type EfiInstallProtocolInterface = unsafe extern "efiapi" fn(
    handle_in_out: *mut EfiHandle,
    protocol: *const EfiGuid,
    interface_type: EfiInterfaceType,
    interface: *const Void,
) -> EfiStatus;
type EfiReinstallProtocolInterface = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    old_interface: *const Void,
    new_interface: *const Void,
) -> EfiStatus;
type EfiUninstallProtocolInterface = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    interface: *const Void,
) -> EfiStatus;
type EfiHandleProtocol = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    interface_out: *mut *const Void,
) -> EfiStatus;
type EfiRegisterProtocolNotify = unsafe extern "efiapi" fn(
    protocol: *const EfiGuid,
    event: EfiEvent,
    registration_out: *mut *const Void,
) -> EfiStatus;
type EfiLocateHandle = unsafe extern "efiapi" fn(
    search_type: EfiLocateSearchType,
    protocol_opttional: *const EfiGuid,
    search_key_optional: *const Void,
    buffer_size_out: *mut UnsignedIntNative,
    buffer_out: *mut EfiHandle,
) -> EfiStatus;
type EfiLocateDevicePath = unsafe extern "efiapi" fn(
    protocol: *const EfiGuid,
    device_path: *const *const EfiDevicePathProtocol,
    device_out: *mut EfiHandle,
) -> EfiStatus;
type EfiInstallConfigurationTable =
    unsafe extern "efiapi" fn(guid: *const EfiGuid, table: *const Void) -> EfiStatus;

type EfiImageLoad = unsafe extern "efiapi" fn(
    boot_policy: Boolean,
    parent_image_handle: EfiHandle,
    device_path_optional: *const EfiDevicePathProtocol,
    source_buffer_optional: *const Void,
    source_size: UnsignedIntNative,
    image_handle_out: *mut EfiHandle,
) -> EfiStatus;
type EfiImageStart = unsafe extern "efiapi" fn(
    image_handle: EfiHandle,
    exit_data_size_out: *mut UnsignedIntNative,
    exit_data_out_optional: *mut *const Char16,
) -> EfiStatus;
type EfiExit = unsafe extern "efiapi" fn(
    image_handle: EfiHandle,
    exit_status: EfiStatus,
    exit_data_size: UnsignedIntNative,
    exit_data_optional: *const Char16,
) -> EfiStatus;
type EfiImageUnload = unsafe extern "efiapi" fn(image_handle: EfiHandle) -> EfiStatus;
type EfiExitBootServices =
    unsafe extern "efiapi" fn(image_handle: EfiHandle, map_key: UnsignedIntNative) -> EfiStatus;

type EfiGetNextMonotonicCount =
    unsafe extern "efiapi" fn(count_out: *mut UnsignedInt64) -> EfiStatus;
type EfiStall = unsafe extern "efiapi" fn(microseconds: UnsignedIntNative) -> EfiStatus;
type EfiSetWatchdogTimer = unsafe extern "efiapi" fn(
    timeout: UnsignedIntNative,
    watchdog_code: UnsignedInt64,
    data_size: UnsignedIntNative,
    watchdog_data_optional: *const Char16,
) -> EfiStatus;

type EfiConnectController = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    driver_image_handle_optional: *const EfiHandle,
    remaining_device_path_optional: *const EfiDevicePathProtocol,
    recursive: Boolean,
) -> EfiStatus;
type EfiDisconnectController = unsafe extern "efiapi" fn(
    controller_handle: EfiHandle,
    driver_image_handle_optional: EfiHandle,
    child_handle_optional: EfiHandle,
) -> EfiStatus;

type EfiOpenProtocol = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    interface_out_optional: *mut *const Void,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
    attributes: UnsignedInt32,
) -> EfiStatus;
type EfiCloseProtocol = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    agent_handle: EfiHandle,
    controller_handle: EfiHandle,
) -> EfiStatus;
type EfiOpenProtocolInformation = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol: *const EfiGuid,
    entry_buffer_out: *mut *const EfiOpenProtocolInformationEntry,
    entry_count_out: *mut UnsignedIntNative,
) -> EfiStatus;

type EfiProtocolsPerHandle = unsafe extern "efiapi" fn(
    handle: EfiHandle,
    protocol_buffer_out: *mut *const *const EfiGuid,
    protocol_buffer_count: *mut UnsignedIntNative,
) -> EfiStatus;
type EfiLocateHandleBuffer = unsafe extern "efiapi" fn(
    search_type: EfiLocateSearchType,
    protocol_optional: *const EfiGuid,
    search_key_optional: *const Void,
    no_handles_out: *mut UnsignedIntNative,
    buffer_out: *mut *const EfiHandle,
) -> EfiStatus;
type EfiLocateProtocol = unsafe extern "efiapi" fn(
    protocol: *const EfiGuid,
    registration_optional: *const Void,
    interface_out: *mut *const Void,
) -> EfiStatus;
type EfiInstallMultipleProtocolInterfaces = unsafe extern "efiapi" fn(
    handle_in_out: *mut EfiHandle,
    c_var_args: CVariableLengthArgument,
) -> EfiStatus;
type EfiUninstallMultipleProtocolInterfaces =
    unsafe extern "efiapi" fn(handle: EfiHandle, c_var_args: CVariableLengthArgument) -> EfiStatus;

type EfiCalculateCrc32 = unsafe extern "efiapi" fn(
    data: *const Void,
    data_size: UnsignedIntNative,
    crc32_out: *mut UnsignedInt32,
) -> EfiStatus;

type EfiCopyMem = unsafe extern "efiapi" fn(
    destination: *const Void,
    source: *const Void,
    length: UnsignedIntNative,
) -> EfiStatus;
type EfiSetMem = unsafe extern "efiapi" fn(
    buffer: *const Void,
    size: UnsignedIntNative,
    value: UnsignedInt8,
) -> EfiStatus;
type EfiCreateEventEx = unsafe extern "efiapi" fn(
    r#type: UnsignedInt32,
    notify_tpl: EfiTpl,
    notify_function_optional: Option<EfiEventNotify>,
    notify_context_optional: *const Void,
    event_group_optional: *const EfiGuid,
    event_out: *mut EfiEvent,
) -> EfiStatus;

/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-boot-services
#[repr(C)]
pub struct EfiBootServices {
    hdr: EfiTableHeader,

    raise_tpl: EfiRaiseTpl,
    restore_tpl: EfiRestoreTpl,

    allocate_pages: EfiAllocatePages,
    free_pages: EfiFreePages,
    get_memory_map: EfiGetMemoryMap,
    allocate_pool: EfiAllocatePool,
    free_pool: EfiFreePool,

    create_event: EfiCreateEvent,
    set_timer: EfiSetTimer,
    wait_for_event: EfiWaitForEvent,
    signal_event: EfiSignalEvent,
    close_event: EfiCloseEvent,
    check_event: EfiCheckEvent,

    install_protocol_interface: EfiInstallProtocolInterface,
    reinstall_protocol_interface: EfiReinstallProtocolInterface,
    uninstall_protocol_interface: EfiUninstallProtocolInterface,
    handle_protocol: EfiHandleProtocol,
    reserved: *const Void,
    register_protocol_notify: EfiRegisterProtocolNotify,
    locate_handle: EfiLocateHandle,
    locate_device_path: EfiLocateDevicePath,
    install_configuration_table: EfiInstallConfigurationTable,

    load_image: EfiImageLoad,
    start_image: EfiImageStart,
    exit: EfiExit,
    unload_image: EfiImageUnload,
    exit_boot_services: EfiExitBootServices,

    get_next_monotonic_count: EfiGetNextMonotonicCount,
    stall: EfiStall,
    set_watchdog_timer: EfiSetWatchdogTimer,

    connect_controller: EfiConnectController,
    disconnect_controller: EfiDisconnectController,

    open_protocol: EfiOpenProtocol,
    close_protocol: EfiCloseProtocol,
    open_protocol_information: EfiOpenProtocolInformation,

    protocols_per_handle: EfiProtocolsPerHandle,
    locate_handle_buffer: EfiLocateHandleBuffer,
    locate_protocol: EfiLocateProtocol,
    install_multiple_protocol_interfaces: EfiInstallMultipleProtocolInterfaces,
    uninstall_multiple_protocol_interfaces: EfiUninstallMultipleProtocolInterfaces,

    calculate_crc32: EfiCalculateCrc32,

    copy_mem: EfiCopyMem,
    set_mem: EfiSetMem,
    create_event_ex: EfiCreateEventEx,
}

impl EfiBootServices {
    pub fn allocate_pages(
        &self,
        r#type: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: UnsignedIntNative,
        memory_in_out: &mut EfiPhysicalAddress,
    ) -> EfiStatus {
        let status = unsafe { (self.allocate_pages)(r#type, memory_type, pages, memory_in_out) };
        status
    }

    pub fn get_memory_map(
        &self,
        memory_map_size_in_out: &mut UnsignedIntNative,
        memory_map_out: &mut [UnsignedInt8],
    ) -> Result<(UnsignedIntNative, UnsignedIntNative, UnsignedInt32), EfiStatus> {
        let mut map_key_out = 0;
        let mut descriptor_size_out = 0;
        let mut descriptor_version_out = 0;
        let status = unsafe {
            (self.get_memory_map)(
                memory_map_size_in_out,
                memory_map_out.as_ptr() as *mut EfiMemoryDescriptor,
                &mut map_key_out,
                &mut descriptor_size_out,
                &mut descriptor_version_out,
            )
        };
        match status {
            EFI_SUCCESS => Ok((map_key_out, descriptor_size_out, descriptor_version_out)),
            v => Err(v),
        }
    }
    pub fn allocate_pool(
        &self,
        pool_type: EfiMemoryType,
        size: UnsignedIntNative,
    ) -> Result<&mut [UnsignedInt8], EfiStatus> {
        let mut buffer = null();
        let status = unsafe { (self.allocate_pool)(pool_type, size, &mut buffer) };
        match status {
            EFI_SUCCESS => {
                Ok(unsafe { slice::from_raw_parts_mut(buffer as *mut UnsignedInt8, size) })
            }
            v => Err(v),
        }
    }
    pub fn free_pool(&self, buffer: &[UnsignedInt8]) -> EfiStatus {
        let status = unsafe { (self.free_pool)(buffer.as_ptr() as *const Void) };
        status
    }

    pub fn locate_handle_buffer(
        &self,
        search_type: EfiLocateSearchType,
        protocol_optional: Option<&EfiGuid>,
        search_key_optional: Option<&Void>,
    ) -> Result<(UnsignedIntNative, &[EfiHandle]), EfiStatus> {
        let mut no_handles = 0;
        let mut buffer = null();
        let status = unsafe {
            (self.locate_handle_buffer)(
                search_type,
                match protocol_optional {
                    Some(protocol) => protocol,
                    None => null(),
                },
                match search_key_optional {
                    Some(search_key) => search_key,
                    None => null(),
                },
                &mut no_handles,
                &mut buffer,
            )
        };
        match status {
            EFI_SUCCESS => Ok((no_handles, unsafe {
                slice::from_raw_parts(buffer, no_handles)
            })),
            v => Err(v),
        }
    }

    pub fn exit_boot_services(
        &self,
        image_handle: EfiHandle,
        map_key: UnsignedIntNative,
    ) -> EfiStatus {
        let status = unsafe { (self.exit_boot_services)(image_handle, map_key) };
        status
    }

    pub fn open_protocol<T>(
        &self,
        handle: EfiHandle,
        protocol: &EfiGuid,
        interface_optional: Option<()>,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: UnsignedInt32,
    ) -> Result<Option<&T>, EfiStatus> {
        match interface_optional {
            Some(()) => {
                let mut interface_out = null();
                let status = unsafe {
                    (self.open_protocol)(
                        handle,
                        protocol,
                        &mut interface_out,
                        agent_handle,
                        controller_handle,
                        attributes,
                    )
                };
                match status {
                    EFI_SUCCESS => Ok(Some(
                        unsafe { (interface_out as *const T).as_ref() }.unwrap(),
                    )),
                    v => Err(v),
                }
            }
            None => {
                let status = unsafe {
                    (self.open_protocol)(
                        handle,
                        protocol,
                        null_mut(),
                        agent_handle,
                        controller_handle,
                        attributes,
                    )
                };
                match status {
                    EFI_SUCCESS => Ok(None),
                    v => Err(v),
                }
            }
        }
    }
}
