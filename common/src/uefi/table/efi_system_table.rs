use crate::uefi::{
    data_type::basic_type::{Char16, EfiHandle, UnsignedInt32, UnsignedIntNative},
    protocol::{
        efi_simple_text_input_protocol::EfiSimpleTextInputProtocol,
        efi_simple_text_output_protocol::EfiSimpleTextOutputProtocol,
    },
};

use super::{
    efi_boot_services::EfiBootServices, efi_configuration_table::EfiConfigurationTable,
    efi_runtime_services::EfiRuntimeServices, efi_table_header::EfiTableHeader,
};

/// Documentation is on:
/// https://uefi.org/specs/UEFI/2.10/04_EFI_System_Table.html#id6
#[repr(C)]
pub struct EfiSystemTable {
    hdr: EfiTableHeader,
    firmware_vendor: *const Char16,
    firmware_revision: UnsignedInt32,
    console_in_handle: EfiHandle,
    con_in: *const EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *const EfiSimpleTextOutputProtocol,
    standard_error_handle: EfiHandle,
    std_err: *const EfiSimpleTextOutputProtocol,
    runtime_services: *const EfiRuntimeServices,
    boot_services: *const EfiBootServices,
    number_of_table_entries: UnsignedIntNative,
    econfiguration_table: *const EfiConfigurationTable,
}

impl EfiSystemTable {
    pub fn con_out(&self) -> &EfiSimpleTextOutputProtocol {
        unsafe { &*self.con_out }
    }

    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*self.boot_services }
    }

    pub fn runtime_services(&self) -> &EfiRuntimeServices {
        unsafe { &*self.runtime_services }
    }
}
