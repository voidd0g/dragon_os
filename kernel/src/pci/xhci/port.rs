use crate::services::Services;

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

    pub fn reset_port(&self, services: &Services) -> Result<(), ()> {
        let port_status_and_control_register = self.port_status_and_control_register();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (port_status_and_control_register & 0x0E00_C3E0) + 0x0002_0010;

        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.port_status_and_control_register() & 0x0000_0010 != 0 {
                break 'a Ok(());
            }
        }
    }
}
