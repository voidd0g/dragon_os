#[repr(C)]
pub struct InterruptDescriptor {
    data: [u32; 4],
}

impl InterruptDescriptor {
    pub const fn new(
        offset: u64,
        segment_selector: u16,
        interrupt_stack_table: u8,
        r#type: u8,
        descriptor_privilege_level: u8,
        present: bool,
    ) -> Self {
        Self {
            data: [
                ((offset & 0xFFFF) as u32) + ((segment_selector as u32) << 16),
                ((offset & 0xFFFF_0000) as u32)
                    + (if present { 1 } else { 0 } << 15)
                    + ((descriptor_privilege_level as u32 & 0x3) << 13)
                    + ((r#type as u32 & 0xF) << 8)
                    + (interrupt_stack_table as u32 & 0x7),
                (offset >> 32) as u32,
                0,
            ],
        }
    }
}
