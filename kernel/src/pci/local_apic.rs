use crate::util::get_unsigned_int_8s;

const LOCAL_APIC_ID: usize = 0xFEE0_0020;

pub fn local_apic_id() -> u8 {
    get_unsigned_int_8s(unsafe { (LOCAL_APIC_ID as *const u32).read() }).3
}
