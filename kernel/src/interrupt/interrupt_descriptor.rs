#[repr(C)]
pub struct InterruptDescriptor {
    offset_low: u16,
    segment_selector: u16,
    attibutes: u16,
    offset_middle: u16,
    offset_high: u32,
    reserved: u32,
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
            offset_low: offset as u16,
            segment_selector,
            attibutes: if present { 0x80_00 } else { 0x00_00 }
                + ((descriptor_privilege_level as u16 & 0x00_07) << 12)
                + ((r#type as u16 & 0x00_0F) << 8)
                + (interrupt_stack_table as u16 & 0x00_07),
            offset_middle: (offset >> u16::BITS) as u16,
            offset_high: (offset >> (u16::BITS + u16::BITS)) as u32,
            reserved: 0,
        }
    }
}
