use core::{
    arch::{asm, global_asm},
    ptr::addr_of_mut,
};

use crate::queue::Queue;

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

pub enum InterruptMessage {
    XhciInterrupt,
}

const QUEUE_BUF_INIT_VAL: Option<InterruptMessage> = None;
const QUEUE_BUF_SIZE: usize = 256;
static mut QUEUE_BUF: [Option<InterruptMessage>; QUEUE_BUF_SIZE] =
    [QUEUE_BUF_INIT_VAL; QUEUE_BUF_SIZE];
static mut INTERRUPT_QUEUE: Queue<InterruptMessage> =
    Queue::new(unsafe { QUEUE_BUF.as_mut_ptr() }, QUEUE_BUF_SIZE);

pub fn push_interrupt_queue(interrupt_message: InterruptMessage) -> Result<(), ()> {
    unsafe { INTERRUPT_QUEUE.push(interrupt_message) }
}
pub fn front_interrupt_queue() -> &'static Option<InterruptMessage> {
    unsafe { INTERRUPT_QUEUE.front() }
}
pub fn pop_interrupt_queue() -> Option<InterruptMessage> {
    unsafe { INTERRUPT_QUEUE.pop() }
}
pub fn count_interrupt_queue() -> usize {
    unsafe { INTERRUPT_QUEUE.count() }
}
