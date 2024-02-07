#![no_std]
#![no_main]

use core::panic::PanicInfo;
use common::uefi::{data_types::{EFI_HANDLE, EFI_STATUS}, system_table::SystemTable};
use utf16_literal::utf16;

#[no_mangle]
pub extern "C" fn efi_main(image_handle: EFI_HANDLE, system_table: &SystemTable) -> EFI_STATUS {
    let cout = system_table.con_out();
    match cout.reset(false) {
        EFI_STATUS::Success => (),
        e => return e,
    }
    match cout.output_string(utf16!("Hello World\r\na\nb\rc\nd\r\n")) {
        EFI_STATUS::Success => (),
        e => return e,
    }

    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}