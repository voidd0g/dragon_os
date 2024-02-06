#![no_std]
#![no_main]

use core::panic::PanicInfo;
use common::uefi::{data_types::{EFI_HANDLE, EFI_STATUS}, system_table::SystemTable};
use utf16_literal::utf16;

#[no_mangle]
pub extern "C" fn efi_main(ImageHandle: EFI_HANDLE, SystemTable: &SystemTable) -> EFI_STATUS {
    let cout = SystemTable.ConOut();
    _ = cout.Reset(false);
    _ = cout.OutputString(utf16!("Hello World\r\na\nb\rc\nd\r\n").as_ptr());

    loop{}

    EFI_STATUS::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}