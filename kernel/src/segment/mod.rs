use core::{
    arch::{asm, global_asm},
    mem::size_of,
    ptr::addr_of,
};

use self::{
    descriptor_type::{DESCRIPTOR_TYPE_EXECUTE_READ, DESCRIPTOR_TYPE_READ_WRITE},
    segment_descriptor::SegmentDescriptor,
};

pub mod descriptor_type;
pub mod segment_descriptor;

const GLOBAL_DESCRIPTOR_TABLE_RESET_VALUE: SegmentDescriptor = SegmentDescriptor::new(
    0, 0, 0, 0, false, 0, false, 0, false, false, false, false, 0,
);
const GLOBAL_DESCRIPTOR_TABLE_COUNT: usize = 3;
static mut GLOBAL_DESCRIPTOR_TABLE: [SegmentDescriptor; GLOBAL_DESCRIPTOR_TABLE_COUNT] =
    [GLOBAL_DESCRIPTOR_TABLE_RESET_VALUE; GLOBAL_DESCRIPTOR_TABLE_COUNT];

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
    const CODE_SEGMENT: usize = 1;
    const STACK_SEGMENT: usize = 2;
    unsafe {
        GLOBAL_DESCRIPTOR_TABLE[CODE_SEGMENT] =
            code_segment(DESCRIPTOR_TYPE_EXECUTE_READ, 0, 0, 0x000F_FFFF);
        GLOBAL_DESCRIPTOR_TABLE[STACK_SEGMENT] =
            data_segment(DESCRIPTOR_TYPE_READ_WRITE, 0, 0, 0x000F_FFFF);
        load_gdt(
            (GLOBAL_DESCRIPTOR_TABLE_COUNT * size_of::<SegmentDescriptor>() - 1) as u16,
            GLOBAL_DESCRIPTOR_TABLE.as_ptr() as u64,
        );
        set_segment_registers_unused(0);
        set_cs_and_ss(
            (CODE_SEGMENT * size_of::<SegmentDescriptor>()) as u16,
            (STACK_SEGMENT * size_of::<SegmentDescriptor>()) as u16,
        );
    }
}

extern "C" {
    fn load_gdt(size_minus_one: u16, head_address: u64);
    fn set_segment_registers_unused(val: u16);
    fn set_cs_and_ss(cs: u16, ss: u16);
}
global_asm!(
    r#"
load_gdt:
	push rbp
	mov rbp, rsp
	sub rsp, 10
	mov [rsp], di
	mov [rsp + 2], rsi
	lgdt [rsp]
	mov rsp, rbp
	pop rbp
	ret

set_segment_registers_unused:
    mov ds, di
    mov es, di
    mov fs, di
    mov gs, di
	ret

set_cs_and_ss:
    mov ss, si
    lea rax, .next[rip]
    push rdi
    push rax
    retfq
.next:
    ret
"#
);
