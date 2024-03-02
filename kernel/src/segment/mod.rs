use core::arch::global_asm;

use self::{descriptor_type::{DESCRIPTOR_TYPE_EXECUTE_READ, DESCRIPTOR_TYPE_READ_WRITE}, segment_descriptor::SegmentDescriptor};

pub mod descriptor_type;
pub mod segment_descriptor;

const GLOBAL_DESCRIPTOR_TABLE_RESET_VALUE: SegmentDescriptor = SegmentDescriptor::new(
    0, 0, 0, 0, false, 0, false, 0, false, false, false, false, 0,
);
static mut GLOBAL_DESCRIPTOR_TABLE: [SegmentDescriptor; 3] =
    [GLOBAL_DESCRIPTOR_TABLE_RESET_VALUE; 3];

fn code_segment(
    r#type: u8,
    descriptor_privilege_level: u8,
    base: u32,
    limit: u32,
) -> SegmentDescriptor {
    SegmentDescriptor::new(
        limit as u16,
        base as u16,
        (base >> 16) as u8,
        r#type,
        true,
        descriptor_privilege_level,
        true,
        (limit >> 16) as u8,
        false,
        true,
        false,
        true,
        (base >> 24) as u8,
    )
}

fn data_segment(
    r#type: u8,
    descriptor_privilege_level: u8,
    base: u32,
    limit: u32,
) -> SegmentDescriptor {
    SegmentDescriptor::new(
        limit as u16,
        base as u16,
        (base >> 16) as u8,
        r#type,
        true,
        descriptor_privilege_level,
        true,
        (limit >> 16) as u8,
        false,
        false,
        true,
        true,
        (base >> 24) as u8,
    )
}

pub fn setup_segments() {
    unsafe {
        GLOBAL_DESCRIPTOR_TABLE[1] = code_segment(DESCRIPTOR_TYPE_EXECUTE_READ, 0, 0, 0x000F_FFFF);
        GLOBAL_DESCRIPTOR_TABLE[2] = data_segment(DESCRIPTOR_TYPE_READ_WRITE, 0, 0, 0x000F_FFFF);
    }
}

extern "C" {
    fn load_gdt(size_minus_one: u16, head_address: u64);
}
global_asm!(
    r#"
load_gdt:
	push rbp
	mov rbp, rsp
	sub rsp, 10
	mov [rsp], di,
	mov [rsp + 2], rsi
	lgdt [rsp]
	mov rsp, rbp
	pop rbp
	ret
"#
);