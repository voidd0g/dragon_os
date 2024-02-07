#![no_std]
#![no_main]

use core::panic::PanicInfo;
use common::uefi::{data_types::common_types::{EFI_HANDLE, EFI_STATUS, EFI_SUCCESS}, tables::efi_system_table::EFI_SYSTEM_TABLE};
use utf16_literal::utf16;

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let cout = system_table.con_out();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => return v,
    }
    match cout.output_string(utf16!("Hello World\r\na\nb\rc\nd\r\n")) {
        EFI_SUCCESS => (),
        v => return v,
    }

    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}