use core::{
    arch::global_asm,
    mem::{size_of, swap},
    ptr::addr_of_mut,
};

use crate::segment::descriptor_type::DESCRIPTOR_TYPE_INTERRUPT_GATE;

use self::{
    interrupt_descriptor::InterruptDescriptor,
    interrupt_vector::{
        INTERRUPT_VECTOR_XHCI_SLOT_0, INTERRUPT_VECTOR_XHCI_SLOT_1, INTERRUPT_VECTOR_XHCI_SLOT_2,
        INTERRUPT_VECTOR_XHCI_SLOT_3,
    },
};

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
        INTERRUPT_VECTOR_XHCI_SLOT_0,
        xhci_interrupt_slot_0_handler as u64,
        DESCRIPTOR_TYPE_INTERRUPT_GATE,
        0,
    );
    idt.set_idt_entry(
        INTERRUPT_VECTOR_XHCI_SLOT_1,
        xhci_interrupt_slot_1_handler as u64,
        DESCRIPTOR_TYPE_INTERRUPT_GATE,
        0,
    );
    idt.set_idt_entry(
        INTERRUPT_VECTOR_XHCI_SLOT_2,
        xhci_interrupt_slot_2_handler as u64,
        DESCRIPTOR_TYPE_INTERRUPT_GATE,
        0,
    );
    idt.set_idt_entry(
        INTERRUPT_VECTOR_XHCI_SLOT_3,
        xhci_interrupt_slot_3_handler as u64,
        DESCRIPTOR_TYPE_INTERRUPT_GATE,
        0,
    );
    idt.load();
}

fn notify_end_of_interrupt() {
    *unsafe { (0xFEE0_00B0 as *mut u32).as_mut() }.unwrap() = 0
}

extern "x86-interrupt" fn xhci_interrupt_slot_0_handler(_: *const InterruptFrame) {
    _ = push_interrupt_queue(InterruptMessage::XhciInterrupt(0));
    notify_end_of_interrupt();
}
extern "x86-interrupt" fn xhci_interrupt_slot_1_handler(_: *const InterruptFrame) {
    _ = push_interrupt_queue(InterruptMessage::XhciInterrupt(1));
    notify_end_of_interrupt();
}
extern "x86-interrupt" fn xhci_interrupt_slot_2_handler(_: *const InterruptFrame) {
    _ = push_interrupt_queue(InterruptMessage::XhciInterrupt(2));
    notify_end_of_interrupt();
}
extern "x86-interrupt" fn xhci_interrupt_slot_3_handler(_: *const InterruptFrame) {
    _ = push_interrupt_queue(InterruptMessage::XhciInterrupt(3));
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
    XhciInterrupt(usize),
}

pub struct FixedSizeInterruptMessageQueue<const COUNT: usize> {
    buf: [Option<InterruptMessage>; COUNT],
    count: usize,
    tail: usize,
    head: usize,
}

impl<const COUNT: usize> FixedSizeInterruptMessageQueue<COUNT> {
    pub const fn new() -> Self {
        const RESET_VALUE: Option<InterruptMessage> = None;
        Self {
            buf: [RESET_VALUE; COUNT],
            count: 0,
            tail: 0,
            head: 0,
        }
    }

    pub fn pop(&mut self) -> Option<InterruptMessage> {
        match self.buf[self.tail] {
            None => None,
            Some(_) => {
                let mut prev_val = None;
                swap(&mut prev_val, &mut self.buf[self.tail]);
                self.tail = (self.tail + 1) % COUNT;
                self.count -= 1;
                prev_val
            }
        }
    }

    pub fn front(&self) -> &Option<InterruptMessage> {
        &self.buf[self.tail]
    }

    pub fn push(&mut self, v: InterruptMessage) -> Result<(), ()> {
        match self.buf[self.head] {
            Some(_) => Err(()),
            None => {
                self.buf[self.head] = Some(v);
                self.head = (self.head + 1) % COUNT;
                self.count += 1;
                Ok(())
            }
        }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

const QUEUE_BUF_SIZE: usize = 256;
static mut INTERRUPT_QUEUE: FixedSizeInterruptMessageQueue<QUEUE_BUF_SIZE> =
    FixedSizeInterruptMessageQueue::new();

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
