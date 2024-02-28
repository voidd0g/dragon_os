use core::{
    arch::{asm, global_asm},
    ptr::addr_of_mut,
};

use self::interrupt_descriptor::InterruptDescriptor;

pub mod interrupt_descriptor;
pub mod interrupt_descriptor_type;
pub mod interrupt_vector;

static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::new();
const IDT_SIZE: usize = 256;

pub struct InterruptDescriptorTable {
    interrupt_descriptor_table: [InterruptDescriptor; IDT_SIZE],
}

impl InterruptDescriptorTable {
    const fn new() -> Self {
        const INTERRUPT_DESCRIPTOR_RESET: InterruptDescriptor =
            InterruptDescriptor::new(0, 0, 0, 0, 0, false);
        Self {
            interrupt_descriptor_table: [INTERRUPT_DESCRIPTOR_RESET; IDT_SIZE],
        }
    }

    pub fn set_idt_entry(
        &mut self,
        index: u8,
        offset: u64,
        r#type: u8,
        descriptor_privilege_level: u8,
    ) {
        self.interrupt_descriptor_table[index as usize] = InterruptDescriptor::new(
            offset,
            get_cs(),
            0,
            r#type,
            descriptor_privilege_level,
            true,
        );
    }

    pub fn load(&self) {
        unsafe {
            load_idt(
                (IDT_SIZE - 1) as u16,
                self.interrupt_descriptor_table.as_ptr() as usize as u64,
            )
        };
    }
}

extern "C" {
    fn load_idt(size_minus_one: u16, head_address: u64);
}
global_asm!(
    r#"
load_idt:
	push rbp
	mov rbp, rsp
	sub rsp, 10
	mov [rsp], di
	mov [rsp + 2], rsi
	lidt [rsp]
	mov rsp, rbp
	pop rbp
	ret
"#
);

pub fn get_interrupt_descriptor_table() -> &'static mut InterruptDescriptorTable {
    unsafe { addr_of_mut!(INTERRUPT_DESCRIPTOR_TABLE).as_mut() }.unwrap()
}

fn get_cs() -> u16 {
    let ret: u64;
    unsafe { asm!("xor eax, eax", "mov ax, cs", out("eax") ret) }
    ret as u16
}
