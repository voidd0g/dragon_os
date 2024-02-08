use crate::uefi::{data_types::basic_types::{CHAR16, EFI_HANDLE, UINT32, UINTN}, protocols::{efi_simple_text_input_protocol::EFI_SIMPLE_TEXT_INPUT_PROTOCOL, efi_simple_text_output_protocol::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL}};

use super::{efi_boot_services::EFI_BOOT_SERVICES, efi_configuration_table::EFI_CONFIGURATION_TABLE, efi_runtime_services::EFI_RUNTIME_SERVICES, efi_table_header::EFI_TABLE_HEADER};

/// Documentation is on: 
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id6
#[repr(C)]
pub struct EFI_SYSTEM_TABLE {
    Hdr:                    EFI_TABLE_HEADER,
    FirmwareVendor:         *const CHAR16,
    FirmwareRevision:       UINT32,
    ConsoleInHandle:        EFI_HANDLE,
    ConIn:                  *const EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle:       EFI_HANDLE,
    ConOut:                 *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    StandardErrorHandle:    EFI_HANDLE,
    StdErr:                 *const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices:        *const EFI_RUNTIME_SERVICES,
    BootServices:           *const EFI_BOOT_SERVICES,
    NumberOfTableEntries:   UINTN,
    EConfigurationTable:    *const EFI_CONFIGURATION_TABLE,
}

#[deny(non_snake_case)]
impl EFI_SYSTEM_TABLE {
    pub fn con_out(&self) -> &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
        unsafe { 
			&*self.ConOut 
		}
    }

	pub fn boot_services(&self) -> &EFI_BOOT_SERVICES {
		unsafe {
			&*self.BootServices
		}
	}
}