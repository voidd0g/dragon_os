use core::{
    ptr::{null, null_mut},
    slice,
};

use crate::uefi::{
    constant::efi_status::EFI_SUCCESS,
    data_type::{
        basic_type::{
            Boolean, Char16, C_VARIABLE_ARGUMENT, EFI_ALLOCATE_TYPE, EFI_EVENT, EFI_GUID,
            EfiHandle, EFI_INTERFACE_TYPE, EFI_LOCATE_SEARCH_TYPE, EFI_MEMORY_TYPE,
            EFI_PHYSICAL_ADDRESS, EfiStatus, EFI_TIMER_DELAY, EFI_TPL, UnsignedInt32, UnsignedInt64, UnsignedInt8,
            UnsignedIntNative, Void,
        },
        {
            efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR,
            efi_open_protocol_information_entry::EFI_OPEN_PROTOCOL_INFORMATION_ENTRY,
        },
    },
    protocol::efi_device_path_protocol::EFI_DEVICE_PATH_PROTOCOL,
};

use super::efi_table_header::EfiTableHeader;

type EFI_RAISE_TPL = unsafe extern "efiapi" fn(NewTpl: EFI_TPL) -> EFI_TPL;
type EFI_RESTORE_TPL = unsafe extern "efiapi" fn(OldTpl: EFI_TPL) -> Void;

type EFI_ALLOCATE_PAGES = unsafe extern "efiapi" fn(
    Type: EFI_ALLOCATE_TYPE,
    MemoryType: EFI_MEMORY_TYPE,
    Pages: UnsignedIntNative,
    MemoryInOut: *mut EFI_PHYSICAL_ADDRESS,
) -> EfiStatus;
type EFI_FREE_PAGES =
    unsafe extern "efiapi" fn(Memory: EFI_PHYSICAL_ADDRESS, Pages: UnsignedIntNative) -> EfiStatus;
type EFI_GET_MEMORY_MAP = unsafe extern "efiapi" fn(
    MemoryMapSizeInOut: *mut UnsignedIntNative,
    MemoryMapOut: *mut EFI_MEMORY_DESCRIPTOR,
    MapKeyOut: *mut UnsignedIntNative,
    DescriptorSizeOut: *mut UnsignedIntNative,
    DescriptorVersionOut: *mut UnsignedInt32,
) -> EfiStatus;
type EFI_ALLOCATE_POOL = unsafe extern "efiapi" fn(
    PoolType: EFI_MEMORY_TYPE,
    Size: UnsignedIntNative,
    BufferOut: *mut *const Void,
) -> EfiStatus;
type EFI_FREE_POOL = unsafe extern "efiapi" fn(Buffer: *const Void) -> EfiStatus;

type EFI_CREATE_EVENT = unsafe extern "efiapi" fn(
    Type: UnsignedInt32,
    NotifyTpl: EFI_TPL,
    NotifyFunctionOptional: Option<EFI_EVENT_NOTIFY>,
    NotifyContextOptional: *const Void,
    EventOut: *mut EFI_EVENT,
) -> EfiStatus;
type EFI_EVENT_NOTIFY = unsafe extern "efiapi" fn(Event: EFI_EVENT, Context: *const Void) -> Void;
type EFI_SET_TIMER = unsafe extern "efiapi" fn(
    Event: EFI_EVENT,
    Type: EFI_TIMER_DELAY,
    TriggerTime: UnsignedInt64,
) -> EfiStatus;
type EFI_WAIT_FOR_EVENT = unsafe extern "efiapi" fn(
    NumberOfEvents: UnsignedIntNative,
    Event: *const EFI_EVENT,
    IndexOut: *mut UnsignedIntNative,
) -> EfiStatus;
type EFI_SIGNAL_EVENT = unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EfiStatus;
type EFI_CLOSE_EVENT = unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EfiStatus;
type EFI_CHECK_EVENT = unsafe extern "efiapi" fn(Event: EFI_EVENT) -> EfiStatus;

type EFI_INSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    HandleInOut: *mut EfiHandle,
    Protocol: *const EFI_GUID,
    InterfaceType: EFI_INTERFACE_TYPE,
    Interface: *const Void,
) -> EfiStatus;
type EFI_REINSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    OldInterface: *const Void,
    NewInterface: *const Void,
) -> EfiStatus;
type EFI_UNINSTALL_PROTOCOL_INTERFACE = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    Interface: *const Void,
) -> EfiStatus;
type EFI_HANDLE_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    InterfaceOut: *mut *const Void,
) -> EfiStatus;
type EFI_REGISTER_PROTOCOL_NOTIFY = unsafe extern "efiapi" fn(
    Protocol: *const EFI_GUID,
    Event: EFI_EVENT,
    RegistrationOut: *mut *const Void,
) -> EfiStatus;
type EFI_LOCATE_HANDLE = unsafe extern "efiapi" fn(
    SearchType: EFI_LOCATE_SEARCH_TYPE,
    ProtocolOpttional: *const EFI_GUID,
    SearchKeyOptional: *const Void,
    BufferSizeOut: *mut UnsignedIntNative,
    BufferOut: *mut EfiHandle,
) -> EfiStatus;
type EFI_LOCATE_DEVICE_PATH = unsafe extern "efiapi" fn(
    Protocol: *const EFI_GUID,
    DevicePath: *const *const EFI_DEVICE_PATH_PROTOCOL,
    DeviceOut: *mut EfiHandle,
) -> EfiStatus;
type EFI_INSTALL_CONFIGURATION_TABLE =
    unsafe extern "efiapi" fn(Guid: *const EFI_GUID, Table: *const Void) -> EfiStatus;

type EFI_IMAGE_LOAD = unsafe extern "efiapi" fn(
    BootPolicy: Boolean,
    ParentImageHandle: EfiHandle,
    DevicePathOptional: *const EFI_DEVICE_PATH_PROTOCOL,
    SourceBufferOptional: *const Void,
    SourceSize: UnsignedIntNative,
    ImageHandleOut: *mut EfiHandle,
) -> EfiStatus;
type EFI_IMAGE_START = unsafe extern "efiapi" fn(
    ImageHandle: EfiHandle,
    ExitDataSizeOut: *mut UnsignedIntNative,
    ExitDataOutOptional: *mut *const Char16,
) -> EfiStatus;
type EFI_EXIT = unsafe extern "efiapi" fn(
    ImageHandle: EfiHandle,
    ExitStatus: EfiStatus,
    ExitDataSize: UnsignedIntNative,
    ExitDataOptional: *const Char16,
) -> EfiStatus;
type EFI_IMAGE_UNLOAD = unsafe extern "efiapi" fn(ImageHandle: EfiHandle) -> EfiStatus;
type EFI_EXIT_BOOT_SERVICES =
    unsafe extern "efiapi" fn(ImageHandle: EfiHandle, MapKey: UnsignedIntNative) -> EfiStatus;

type EFI_GET_NEXT_MONOTONIC_COUNT = unsafe extern "efiapi" fn(CountOut: *mut UnsignedInt64) -> EfiStatus;
type EFI_STALL = unsafe extern "efiapi" fn(Microseconds: UnsignedIntNative) -> EfiStatus;
type EFI_SET_WATCHDOG_TIMER = unsafe extern "efiapi" fn(
    Timeout: UnsignedIntNative,
    WatchdogCode: UnsignedInt64,
    DataSize: UnsignedIntNative,
    WatchdogDataOptional: *const Char16,
) -> EfiStatus;

type EFI_CONNECT_CONTROLLER = unsafe extern "efiapi" fn(
    ControllerHandle: EfiHandle,
    DriverImageHandleOptional: *const EfiHandle,
    RemainingDevicePathOptional: *const EFI_DEVICE_PATH_PROTOCOL,
    Recursive: Boolean,
) -> EfiStatus;
type EFI_DISCONNECT_CONTROLLER = unsafe extern "efiapi" fn(
    ControllerHandle: EfiHandle,
    DriverImageHandleOptional: EfiHandle,
    ChildHandleOptional: EfiHandle,
) -> EfiStatus;

type EFI_OPEN_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    InterfaceOutOptional: *mut *const Void,
    AgentHandle: EfiHandle,
    ControllerHandle: EfiHandle,
    Attributes: UnsignedInt32,
) -> EfiStatus;
type EFI_CLOSE_PROTOCOL = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    AgentHandle: EfiHandle,
    ControllerHandle: EfiHandle,
) -> EfiStatus;
type EFI_OPEN_PROTOCOL_INFORMATION = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    Protocol: *const EFI_GUID,
    EntryBufferOut: *mut *const EFI_OPEN_PROTOCOL_INFORMATION_ENTRY,
    EntryCountOut: *mut UnsignedIntNative,
) -> EfiStatus;

type EFI_PROTOCOLS_PER_HANDLE = unsafe extern "efiapi" fn(
    Handle: EfiHandle,
    ProtocolBufferOut: *mut *const *const EFI_GUID,
    ProtocolBufferCount: *mut UnsignedIntNative,
) -> EfiStatus;
type EFI_LOCATE_HANDLE_BUFFER = unsafe extern "efiapi" fn(
    SearchType: EFI_LOCATE_SEARCH_TYPE,
    ProtocolOptional: *const EFI_GUID,
    SearchKeyOptional: *const Void,
    NoHandlesOut: *mut UnsignedIntNative,
    BufferOut: *mut *const EfiHandle,
) -> EfiStatus;
type EFI_LOCATE_PROTOCOL = unsafe extern "efiapi" fn(
    Protocol: *const EFI_GUID,
    RegistrationOptional: *const Void,
    InterfaceOut: *mut *const Void,
) -> EfiStatus;
type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = unsafe extern "efiapi" fn(
    HandleInOut: *mut EfiHandle,
    c_var_args: C_VARIABLE_ARGUMENT,
) -> EfiStatus;
type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES =
    unsafe extern "efiapi" fn(Handle: EfiHandle, c_var_args: C_VARIABLE_ARGUMENT) -> EfiStatus;

type EFI_CALCULATE_CRC32 = unsafe extern "efiapi" fn(
    Data: *const Void,
    DataSize: UnsignedIntNative,
    Crc32Out: *mut UnsignedInt32,
) -> EfiStatus;

type EFI_COPY_MEM = unsafe extern "efiapi" fn(
    Destination: *const Void,
    Source: *const Void,
    Length: UnsignedIntNative,
) -> EfiStatus;
type EFI_SET_MEM =
    unsafe extern "efiapi" fn(Buffer: *const Void, Size: UnsignedIntNative, Value: UnsignedInt8) -> EfiStatus;
type EFI_CREATE_EVENT_EX = unsafe extern "efiapi" fn(
    Type: UnsignedInt32,
    NotifyTpl: EFI_TPL,
    NotifyFunctionOptional: Option<EFI_EVENT_NOTIFY>,
    NotifyContextOptional: *const Void,
    EventGroupOptional: *const EFI_GUID,
    EventOut: *mut EFI_EVENT,
) -> EfiStatus;

/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#efi-boot-services
#[repr(C)]
pub struct EFI_BOOT_SERVICES {
    Hdr: EfiTableHeader,

    RaiseTPL: EFI_RAISE_TPL,
    RestoreTPL: EFI_RESTORE_TPL,

    AllocatePages: EFI_ALLOCATE_PAGES,
    FreePages: EFI_FREE_PAGES,
    GetMemoryMap: EFI_GET_MEMORY_MAP,
    AllocatePool: EFI_ALLOCATE_POOL,
    FreePool: EFI_FREE_POOL,

    CreateEvent: EFI_CREATE_EVENT,
    SetTimer: EFI_SET_TIMER,
    WaitForEvent: EFI_WAIT_FOR_EVENT,
    SignalEvent: EFI_SIGNAL_EVENT,
    CloseEvent: EFI_CLOSE_EVENT,
    CheckEvent: EFI_CHECK_EVENT,

    InstallProtocolInterface: EFI_INSTALL_PROTOCOL_INTERFACE,
    ReinstallProtocolInterface: EFI_REINSTALL_PROTOCOL_INTERFACE,
    UninstallProtocolInterface: EFI_UNINSTALL_PROTOCOL_INTERFACE,
    HandleProtocol: EFI_HANDLE_PROTOCOL,
    Reserved: *const Void,
    RegisterProtocolNotify: EFI_REGISTER_PROTOCOL_NOTIFY,
    LocateHandle: EFI_LOCATE_HANDLE,
    LocateDevicePath: EFI_LOCATE_DEVICE_PATH,
    InstallConfigurationTable: EFI_INSTALL_CONFIGURATION_TABLE,

    LoadImage: EFI_IMAGE_LOAD,
    StartImage: EFI_IMAGE_START,
    Exit: EFI_EXIT,
    UnloadImage: EFI_IMAGE_UNLOAD,
    ExitBootServices: EFI_EXIT_BOOT_SERVICES,

    GetNextMonotonicCount: EFI_GET_NEXT_MONOTONIC_COUNT,
    Stall: EFI_STALL,
    SetWatchdogTimer: EFI_SET_WATCHDOG_TIMER,

    ConnectController: EFI_CONNECT_CONTROLLER,
    DisconnectController: EFI_DISCONNECT_CONTROLLER,

    OpenProtocol: EFI_OPEN_PROTOCOL,
    CloseProtocol: EFI_CLOSE_PROTOCOL,
    OpenProtocolInformation: EFI_OPEN_PROTOCOL_INFORMATION,

    ProtocolsPerHandle: EFI_PROTOCOLS_PER_HANDLE,
    LocateHandleBuffer: EFI_LOCATE_HANDLE_BUFFER,
    LocateProtocol: EFI_LOCATE_PROTOCOL,
    InstallMultipleProtocolInterfaces: EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES,
    UninstallMultipleProtocolInterfaces: EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES,

    CalculateCrc32: EFI_CALCULATE_CRC32,

    CopyMem: EFI_COPY_MEM,
    SetMem: EFI_SET_MEM,
    CreateEventEx: EFI_CREATE_EVENT_EX,
}

impl EFI_BOOT_SERVICES {
    pub fn allocate_pages(
        &self,
        r#type: EFI_ALLOCATE_TYPE,
        memory_type: EFI_MEMORY_TYPE,
        pages: UnsignedIntNative,
        memory_in_out: &mut EFI_PHYSICAL_ADDRESS,
    ) -> EfiStatus {
        let status = unsafe { (self.AllocatePages)(r#type, memory_type, pages, memory_in_out) };
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
            (self.GetMemoryMap)(
                memory_map_size_in_out,
                memory_map_out.as_ptr() as *mut EFI_MEMORY_DESCRIPTOR,
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
        pool_type: EFI_MEMORY_TYPE,
        size: UnsignedIntNative,
    ) -> Result<&mut [UnsignedInt8], EfiStatus> {
        let mut buffer = null();
        let status = unsafe { (self.AllocatePool)(pool_type, size, &mut buffer) };
        match status {
            EFI_SUCCESS => Ok(unsafe { slice::from_raw_parts_mut(buffer as *mut UnsignedInt8, size) }),
            v => Err(v),
        }
    }
    pub fn free_pool(&self, buffer: &[UnsignedInt8]) -> EfiStatus {
        let status = unsafe { (self.FreePool)(buffer.as_ptr() as *const Void) };
        status
    }

    pub fn locate_handle_buffer(
        &self,
        search_type: EFI_LOCATE_SEARCH_TYPE,
        protocol_optional: Option<&EFI_GUID>,
        search_key_optional: Option<&Void>,
    ) -> Result<(UnsignedIntNative, &[EfiHandle]), EfiStatus> {
        let mut no_handles = 0;
        let mut buffer = null();
        let status = unsafe {
            (self.LocateHandleBuffer)(
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

    pub fn exit_boot_services(&self, image_handle: EfiHandle, map_key: UnsignedIntNative) -> EfiStatus {
        let status = unsafe { (self.ExitBootServices)(image_handle, map_key) };
        status
    }

    pub fn open_protocol<T>(
        &self,
        handle: EfiHandle,
        protocol: &EFI_GUID,
        interface_optional: Option<()>,
        agent_handle: EfiHandle,
        controller_handle: EfiHandle,
        attributes: UnsignedInt32,
    ) -> Result<Option<&T>, EfiStatus> {
        match interface_optional {
            Some(()) => {
                let mut interface_out = null();
                let status = unsafe {
                    (self.OpenProtocol)(
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
                    (self.OpenProtocol)(
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
