use self::{efi_boot_services::EFI_BOOT_SERVICES, efi_configuration_table::EFI_CONFIGURATION_TABLE, efi_runtime_services::EFI_RUNTIME_SERVICES, efi_simple_text_input_protocol::EFI_SIMPLE_TEXT_INPUT_PROTOCOL, efi_simple_text_output_protocol::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, efi_table_header::EFI_TABLE_HEADER};

use super::data_types::{CHAR16, EFI_HANDLE, UINT32, UINTN};

pub mod efi_table_header;
pub mod efi_simple_text_output_protocol;
pub mod efi_simple_text_input_protocol;
pub mod efi_simple_text_input_ex_protocol;
pub mod efi_runtime_services;
pub mod efi_boot_services;
pub mod efi_configuration_table;

#[repr(C)]
pub struct SystemTable {
    Hdr:                    EFI_TABLE_HEADER,
    FirmwareVendor:         *const CHAR16,
    FirmwareRevision:       UINT32,
    ConsoleInHandle:        EFI_HANDLE,
    ConIn:                  *mut EFI_SIMPLE_TEXT_INPUT_PROTOCOL,
    ConsoleOutHandle:       EFI_HANDLE,
    ConOut:                 *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    StandardErrorHandle:    EFI_HANDLE,
    StdErr:                 *mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL,
    RuntimeServices:        *mut EFI_RUNTIME_SERVICES,
    BootServices:           *mut EFI_BOOT_SERVICES,
    NumberOfTableEntries:   UINTN,
    EConfigurationTable:    *mut EFI_CONFIGURATION_TABLE,
}

impl SystemTable {
    pub fn ConOut(&self) -> &mut EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
        unsafe { 
			&mut *self.ConOut 
		}
    }

	pub fn BootServices(&self) -> &mut EFI_BOOT_SERVICES {
		unsafe {
			&mut *self.BootServices
		}
	}
}