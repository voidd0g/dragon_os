use core::ptr::null_mut;

use crate::uefi::{data_types::{basic_types::{BOOLEAN, CHAR16, C_VARIABLE_ARGUMENT, EFI_ALLOCATE_TYPE, EFI_EVENT, EFI_GUID, EFI_HANDLE, EFI_INTERFACE_TYPE, EFI_LOCATE_SEARCH_TYPE, EFI_MEMORY_TYPE, EFI_PHYSICAL_ADDRESS, EFI_STATUS, EFI_TIMER_DELAY, EFI_TPL, UINT32, UINT64, UINT8, UINTN, VOID}, structs::{efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR, efi_open_protocol_information_entry::EFI_OPEN_PROTOCOL_INFORMATION_ENTRY}}, protocols::efi_device_path_protocol::EFI_DEVICE_PATH_PROTOCOL};

use super::efi_table_header::EFI_TABLE_HEADER;


type EFI_RAISE_TPL = extern "C" fn (NewTpl: EFI_TPL) -> EFI_TPL;
type EFI_RESTORE_TPL = extern "C" fn (OldTpl: EFI_TPL) -> VOID;

type EFI_ALLOCATE_PAGES = extern "C" fn (Type: EFI_ALLOCATE_TYPE, MemoryType: EFI_MEMORY_TYPE, Pages: UINTN, MemoryInOut: *mut EFI_PHYSICAL_ADDRESS) -> EFI_STATUS;
type EFI_FREE_PAGES = extern "C" fn (Memory: EFI_PHYSICAL_ADDRESS, Pages: UINTN) -> EFI_STATUS;
type EFI_GET_MEMORY_MAP = extern "C" fn (MemoryMapSizeInOut: *mut UINTN, MemoryMapOut: *mut EFI_MEMORY_DESCRIPTOR, MapKeyOut: *mut UINTN, DescriptorSizeOut: *mut UINTN, DescriptorVersionOut: *mut UINT32) -> EFI_STATUS;
type EFI_ALLOCATE_POOL = extern "C" fn (PoolType: EFI_MEMORY_TYPE, Size: UINTN, BufferOut: *mut *const VOID) -> EFI_STATUS;
type EFI_FREE_POOL = extern "C" fn (Buffer: *const VOID) -> EFI_STATUS;

type EFI_CREATE_EVENT = extern "C" fn (Type: UINT32, NotifyTpl: EFI_TPL, NotifyFunctionOptional: Option<EFI_EVENT_NOTIFY>, NotifyContextOptional: *const VOID, EventOut: *mut EFI_EVENT) -> EFI_STATUS;
type EFI_EVENT_NOTIFY = extern "C" fn (Event: EFI_EVENT, Context: *const VOID) -> VOID;
type EFI_SET_TIMER = extern "C" fn (Event: EFI_EVENT, Type: EFI_TIMER_DELAY, TriggerTime: UINT64) -> EFI_STATUS;
type EFI_WAIT_FOR_EVENT = extern "C" fn (NumberOfEvents: UINTN, Event: *const EFI_EVENT, IndexOut: *mut UINTN) -> EFI_STATUS;
type EFI_SIGNAL_EVENT = extern "C" fn (Event: EFI_EVENT) -> EFI_STATUS;
type EFI_CLOSE_EVENT = extern "C" fn (Event: EFI_EVENT) -> EFI_STATUS;
type EFI_CHECK_EVENT = extern "C" fn (Event: EFI_EVENT) -> EFI_STATUS;

type EFI_INSTALL_PROTOCOL_INTERFACE = extern "C" fn (HandleInOut: *mut EFI_HANDLE, Protocol: *const EFI_GUID, InterfaceType: EFI_INTERFACE_TYPE, Interface: *const VOID) -> EFI_STATUS;
type EFI_REINSTALL_PROTOCOL_INTERFACE = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, OldInterface: *const VOID, NewInterface: *const VOID) -> EFI_STATUS;
type EFI_UNINSTALL_PROTOCOL_INTERFACE = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, Interface: *const VOID) -> EFI_STATUS;
type EFI_HANDLE_PROTOCOL = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, InterfaceOut: *mut *const VOID) -> EFI_STATUS;
type EFI_REGISTER_PROTOCOL_NOTIFY = extern "C" fn (Protocol: *const EFI_GUID, Event: EFI_EVENT, RegistrationOut: *mut *const VOID) -> EFI_STATUS;
type EFI_LOCATE_HANDLE = extern "C" fn (SearchType: EFI_LOCATE_SEARCH_TYPE, ProtocolOpttional: *const EFI_GUID, SearchKeyOptional: *const VOID, BufferSizeOut: *mut UINTN, BufferOut: *mut EFI_HANDLE) -> EFI_STATUS;
type EFI_LOCATE_DEVICE_PATH = extern "C" fn (Protocol: *const EFI_GUID, DevicePath: *const *const EFI_DEVICE_PATH_PROTOCOL, DeviceOut: *mut EFI_HANDLE) -> EFI_STATUS;
type EFI_INSTALL_CONFIGURATION_TABLE = extern "C" fn (Guid: *const EFI_GUID, Table: *const VOID) -> EFI_STATUS;

type EFI_IMAGE_LOAD = extern "C" fn (BootPolicy: BOOLEAN, ParentImageHandle: EFI_HANDLE, DevicePathOptional: *const EFI_DEVICE_PATH_PROTOCOL, SourceBufferOptional: *const VOID, SourceSize: UINTN, ImageHandleOut: *mut EFI_HANDLE) -> EFI_STATUS;
type EFI_IMAGE_START = extern "C" fn (ImageHandle: EFI_HANDLE, ExitDataSizeOut: *mut UINTN, ExitDataOutOptional: *mut *const CHAR16) -> EFI_STATUS;
type EFI_EXIT = extern "C" fn (ImageHandle: EFI_HANDLE, ExitStatus: EFI_STATUS, ExitDataSize: UINTN, ExitDataOptional: *const CHAR16) -> EFI_STATUS;
type EFI_IMAGE_UNLOAD = extern "C" fn (ImageHandle: EFI_HANDLE) -> EFI_STATUS;
type EFI_EXIT_BOOT_SERVICES = extern "C" fn (ImageHandle: EFI_HANDLE, MapKey: UINTN) -> EFI_STATUS;

type EFI_GET_NEXT_MONOTONIC_COUNT = extern "C" fn (CountOut: *mut UINT64) -> EFI_STATUS;
type EFI_STALL = extern "C" fn (Microseconds: UINTN) -> EFI_STATUS;
type EFI_SET_WATCHDOG_TIMER = extern "C" fn (Timeout: UINTN, WatchdogCode: UINT64, DataSize: UINTN, WatchdogDataOptional: *const CHAR16) -> EFI_STATUS;

type EFI_CONNECT_CONTROLLER = extern "C" fn (ControllerHandle: EFI_HANDLE, DriverImageHandleOptional: *const EFI_HANDLE, RemainingDevicePathOptional: *const EFI_DEVICE_PATH_PROTOCOL, Recursive: BOOLEAN) -> EFI_STATUS;
type EFI_DISCONNECT_CONTROLLER = extern "C" fn (ControllerHandle: EFI_HANDLE, DriverImageHandleOptional: EFI_HANDLE, ChildHandleOptional: EFI_HANDLE) -> EFI_STATUS;

type EFI_OPEN_PROTOCOL = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, InterfaceOutOptional: *mut *const VOID, AgentHandle: EFI_HANDLE, ControllerHandle: EFI_HANDLE, Attributes: UINT32) -> EFI_STATUS;
type EFI_CLOSE_PROTOCOL = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, AgentHandle: EFI_HANDLE, ControllerHandle: EFI_HANDLE) -> EFI_STATUS;
type EFI_OPEN_PROTOCOL_INFORMATION = extern "C" fn (Handle: EFI_HANDLE, Protocol: *const EFI_GUID, EntryBufferOut: *mut *const EFI_OPEN_PROTOCOL_INFORMATION_ENTRY, EntryCountOut: *mut UINTN) -> EFI_STATUS;

type EFI_PROTOCOLS_PER_HANDLE = extern "C" fn (Handle: EFI_HANDLE, ProtocolBufferOut: *mut *const *const EFI_GUID, ProtocolBufferCount: *mut UINTN) -> EFI_STATUS;
type EFI_LOCATE_HANDLE_BUFFER = extern "C" fn (SearchType: EFI_LOCATE_SEARCH_TYPE, ProtocolOptional: *const EFI_GUID, SearchKeyOptional: *const VOID, NoHandlesOut: *mut UINTN, BufferOut: *mut *const EFI_HANDLE) -> EFI_STATUS;
type EFI_LOCATE_PROTOCOL = extern "C" fn (Protocol: *const EFI_GUID, RegistrationOptional: *const VOID, InterfaceOut: *mut *const VOID) -> EFI_STATUS;
type EFI_INSTALL_MULTIPLE_PROTOCOL_INTERFACES = extern "C" fn (HandleInOut: *mut EFI_HANDLE, c_var_args: C_VARIABLE_ARGUMENT) -> EFI_STATUS;
type EFI_UNINSTALL_MULTIPLE_PROTOCOL_INTERFACES = extern "C" fn (Handle: EFI_HANDLE, c_var_args: C_VARIABLE_ARGUMENT) -> EFI_STATUS;

type EFI_CALCULATE_CRC32 = extern "C" fn (Data: *const VOID, DataSize: UINTN, Crc32Out: *mut UINT32) -> EFI_STATUS;

type EFI_COPY_MEM = extern "C" fn (Destination: *const VOID, Source: *const VOID, Length: UINTN) -> EFI_STATUS;
type EFI_SET_MEM = extern "C" fn (Buffer: *const VOID, Size: UINTN, Value: UINT8) -> EFI_STATUS;
type EFI_CREATE_EVENT_EX = extern "C" fn (Type: UINT32, NotifyTpl: EFI_TPL, NotifyFunctionOptional: Option<EFI_EVENT_NOTIFY>, NotifyContextOptional: *const VOID, EventGroupOptional: *const EFI_GUID, EventOut: *mut EFI_EVENT) -> EFI_STATUS;

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
	Reserved: *const VOID,
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

#[deny(non_snake_case)]
impl EFI_BOOT_SERVICES {
	pub fn get_memory_map(&self, memory_map_size_in_out: &mut UINTN, memory_map_out: &mut[UINT8], map_key_out: &mut UINTN, descriptor_size_out: &mut UINTN, descriptor_version_out: &mut UINT32) -> EFI_STATUS {
		(self.GetMemoryMap)(memory_map_size_in_out, memory_map_out.as_ptr() as *mut EFI_MEMORY_DESCRIPTOR, map_key_out, descriptor_size_out, descriptor_version_out)
	}

	pub fn open_protocol(&self, handle: EFI_HANDLE, protocol: &EFI_GUID, interface_out_optional: Option<&mut[UINT8]>, agent_handle: EFI_HANDLE, controller_handle: EFI_HANDLE, attributes: UINT32) -> EFI_STATUS {
		(self.OpenProtocol)(handle, protocol, match interface_out_optional {
			Some(interface_out) => &mut (interface_out.as_mut_ptr() as *const VOID) as *mut *const VOID,
			None => null_mut(),
		}, agent_handle, controller_handle, attributes)
	}
}