#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}