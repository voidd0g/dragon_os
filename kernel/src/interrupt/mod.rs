use core::{
    arch::{asm, global_asm},
    mem::{size_of, transmute},
    ptr::addr_of_mut,
};

use crate::{segment::descriptor_type::DESCRIPTOR_TYPE_INTERRUPT_GATE, util::queue::Queue};

use self::{interrupt_descriptor::InterruptDescriptor, interrupt_vector::INTERRUPT_VECTOR_XHCI};

pub mod interrupt_descriptor;
pub mod interrupt_vector;

static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::new();
const IDT_SIZE: usize = 256;

struct InterruptDescriptorTable {
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
        vector: u8,
        offset: u64,
        r#type: u8,
        descriptor_privilege_level: u8,
    ) {
        self.interrupt_descriptor_table[vector as usize] = InterruptDescriptor::new(
            offset,
            unsafe { get_cs() },
            0,
            r#type,
            descriptor_privilege_level,
            true,
        );
    }

    pub fn load(&self) {
        unsafe {
            load_idt(
                (IDT_SIZE * size_of::<InterruptDescriptor>() - 1) as u16,
                self.interrupt_descriptor_table.as_ptr() as u64,
            )
        };
    }
}

extern "C" {
    fn load_idt(size_minus_one: u16, head_address: u64);
    fn get_cs() -> u16;
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

get_cs:
    xor eax, eax
    mov ax, cs
    ret
"#
);

pub fn setup_interrupt_descriptor_table() {
    let idt = unsafe { addr_of_mut!(INTERRUPT_DESCRIPTOR_TABLE).as_mut() }.unwrap();
    idt.set_idt_entry(
        INTERRUPT_VECTOR_XHCI,
        xhci_interrupt_handler as u64,
        DESCRIPTOR_TYPE_INTERRUPT_GATE,
        0,
    );
    idt.load();
}

fn notify_end_of_interrupt() {
    *unsafe { (0xFEE0_00B0 as *mut u32).as_mut() }.unwrap() = 0
}

extern "x86-interrupt" fn xhci_interrupt_handler(_: *const InterruptFrame) {
    _ = push_interrupt_queue(InterruptMessage::XhciInterrupt);
    unsafe { INTERRUPT_OCCURED = true };
    notify_end_of_interrupt();
}

#[repr(C)]
struct InterruptFrame {
    rip: u64,
    cs: u64,
    rflags: u64,
    rsp: u64,
    ss: u64,
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

pub static mut INTERRUPT_OCCURED: bool = false;