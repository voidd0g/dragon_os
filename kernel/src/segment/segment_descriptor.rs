pub struct SegmentDescriptor(u64);
impl SegmentDescriptor {
    pub const fn new(
        limit_low: u16,
        base_low: u16,
        base_middle: u8,
        r#type: u8,
        system_segment: bool,
        descriptor_privilege_level: u8,
        present: bool,
        limit_high: u8,
        available: bool,
        long_mode: bool,
        default_operation_size: bool,
        granularity: bool,
        base_high: u8,
    ) -> Self {
        Self(
            (limit_low as u64)
                + ((base_low as u64) << 16)
                + ((base_middle as u64) << 32)
                + (((r#type & 0x0F) as u64) << 40)
                + (if system_segment { 1 } else { 0 } << 44)
                + (((descriptor_privilege_level & 0x03) as u64) << 45)
                + (if present { 1 } else { 0 } << 47)
                + (((limit_high & 0x0F) as u64) << 48)
                + (if available { 1 } else { 0 } << 52)
                + (if long_mode { 1 } else { 0 } << 53)
                + (if default_operation_size { 1 } else { 0 } << 54)
                + (if granularity { 1 } else { 0 } << 55)
                + ((base_high as u64) << 56),
        )
    }
}
