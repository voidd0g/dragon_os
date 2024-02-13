#![no_std]
#![no_main]

use core::{arch::asm, panic::PanicInfo};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(frame_buffer_base: u64, frame_buffer_size: usize) -> ! {
    let frame_buffer_mut =
        unsafe { slice::from_raw_parts_mut(frame_buffer_base as *mut [u8; 4], frame_buffer_size) };
    let mut frame_buffer_mut_iter = frame_buffer_mut.iter_mut();
    let mut i = 0;
    'a: loop {
        match frame_buffer_mut_iter.next() {
            Some(pixel) => {
                *pixel = [
                    if i < 100000 { i % u8::MAX } else { 0 },
                    if i < 200000 { u8::MAX } else { 0 },
                    if i < 300000 { u8::MAX } else { 0 },
                    0,
                ]
            }
            None => break 'a (),
        }
        i += 1;
    }

    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
