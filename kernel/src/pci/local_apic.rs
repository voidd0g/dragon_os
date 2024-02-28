use crate::util::get_unsigned_int_8s;

const END_OF_INTERRUPT: usize = 0xFEE0_00B0;
const LOCAL_APIC_ID: usize = 0xFEE0_0020;

pub fn notify_end_of_interrupt() {
    unsafe {
        *(END_OF_INTERRUPT as *mut u32) = 0;
    }
}

pub fn local_apic_id() -> u8 {
    get_unsigned_int_8s(unsafe { (LOCAL_APIC_ID as *const u32).read() }).3
}
