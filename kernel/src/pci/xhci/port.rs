pub struct XhciPort {
    base_address: u64,
}

impl XhciPort {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

	pub fn port_status_and_control_register(&self) -> u32 {
		unsafe { ((self.base_address + 0x00) as *const u32).read() }
	}
}
