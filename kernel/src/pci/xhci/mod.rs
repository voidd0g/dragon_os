pub mod command_ring;
pub mod context;
pub mod event_ring;
pub mod port;
pub mod transfer_request_block;

use common::iter_str::{IterStrFormat, ToIterStr};

use crate::{
    font::font_writer::FONT_HEIGHT,
    output_string,
    pixel_writer::pixel_color::PixelColor,
    services::Services,
    util::{get_bits_value, get_unsigned_int_8s, vector2::Vector2},
};

use self::{
    command_ring::CommandRingManager,
    context::{DeviceContextBaseAddressArray, DeviceContexts},
    event_ring::EventRingManagerWithFixedSize,
};

const MAX_DEVICE_SLOTS_DESIRED: u8 = 8;
const DEVICE_CONTEXT_BASE_ADDRESS_ARRAY_COUNT: usize = MAX_DEVICE_SLOTS_DESIRED as usize + 1;
const COMMAND_RING_SIZE: usize = 8;
const PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_COUNT: u16 = 1;
const PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_SIZE: u16 = 64;
pub struct XhcDevice {
    capability_registers: XhcCapabilityRegisters,
    operational_registers: XhcOperationalRegisters,
    device_context_base_address_array:
        DeviceContextBaseAddressArray<DEVICE_CONTEXT_BASE_ADDRESS_ARRAY_COUNT>,
    device_contexts: DeviceContexts<{ MAX_DEVICE_SLOTS_DESIRED as usize }>,
    command_ring: CommandRingManager<COMMAND_RING_SIZE>,
    primary_interrupter_event_ring: EventRingManagerWithFixedSize<
        PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_SIZE,
        PRIMARY_INTERRUPTER_EVENT_RING_SEGMENT_COUNT,
    >,
    runtime_registers: XhcRuntimeRegisters,
    
}

impl XhcDevice {
    pub fn new(base_address: u64) -> Self {
        let capability_registers = XhcCapabilityRegisters::new(base_address);
        let operational_registers_offset = capability_registers.capability_register_length();
        let runtime_registers_offset = capability_registers.runtime_register_space_offset();

        Self {
            capability_registers,
            operational_registers: XhcOperationalRegisters::new(
                base_address + operational_registers_offset as u64,
            ),
            device_context_base_address_array: DeviceContextBaseAddressArray::new(),
            device_contexts: DeviceContexts::new(),
            command_ring: CommandRingManager::new(XhcOperationalRegisters::new(
                base_address + operational_registers_offset as u64,
            )),
            primary_interrupter_event_ring: EventRingManagerWithFixedSize::new(
                XhcRuntimeRegisters::new(base_address + runtime_registers_offset as u64)
                    .get_interrupter_register_set(0),
            ),
            runtime_registers: XhcRuntimeRegisters::new(
                base_address + runtime_registers_offset as u64,
            ),
        }
    }

    pub fn initialize(&self, services: &Services, height: &mut u32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Check USB status host controller halted bit.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        if self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK == 0 {
            self.operational_registers.usb_command_stop();

            'a: loop {
                match services.time_services().wait_for_nano_seconds(1_000_000) {
                    Ok(()) => (),
                    Err(()) => return Err(()),
                }
                if self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK
                    != 0
                {
                    break 'a ();
                }
            }
        }

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Do USB host controller reset.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.operational_registers
            .usb_command_host_controller_reset();

        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_command() & USB_COMMAND_HOST_CONTROLLER_RESET_MASK
                == 0
            {
                break 'a ();
            }
        }
        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_status() & USB_STATUS_CONTROLLER_NOT_READY == 0 {
                break 'a ();
            }
        }

        let max_device_slots = get_bits_value(
            self.capability_registers
                .host_controller_structural_parameters_1(),
            0,
            7,
        ) as u8;
        let slots_enabled = if max_device_slots < MAX_DEVICE_SLOTS_DESIRED {
            max_device_slots
        } else {
            MAX_DEVICE_SLOTS_DESIRED
        };
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"Set max device slots to ".to_iter_str(IterStrFormat::none()),
                slots_enabled.to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.operational_registers
            .set_number_of_device_slots_enabled(slots_enabled);

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set DCBAAP.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.operational_registers
            .set_device_context_base_address_array_pointer(unsafe {
                self.device_context_base_address_array.pointer()
            } as u64);

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set command ring.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        self.command_ring.initialize();

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"Set primary interrupter event ring.".to_iter_str(IterStrFormat::none()),
                self.capability_registers
                    .runtime_register_space_offset()
                    .to_iter_str(IterStrFormat::new(
                        Some(common::iter_str::Radix::Hexadecimal),
                        None,
                        None
                    ))
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.primary_interrupter_event_ring.initialize();

        self.operational_registers.usb_command_interrupter_enable();

        // let context_size = self
        //     .capability_registers
        //     .host_controller_cabability_parameters_1()
        //     & 0x0000_0004
        //     != 0;

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Succeeded in Xhc initialization.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        Ok(())
    }

    pub fn run(&self, services: &Services, height: &mut u32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Run usb.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        self.operational_registers.usb_command_run();
        match services.time_services().wait_for_nano_seconds(1_000_000) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }

        'a: loop {
            match services.time_services().wait_for_nano_seconds(1_000_000) {
                Ok(()) => (),
                Err(()) => return Err(()),
            }
            if self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK == 0
            {
                break 'a ();
            }
        }

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Usb started.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        Ok(())
    }
}

const USB_STATUS_HOST_CONTROLLER_HALTED_MASK: u32 = 0x0000_0001;
const USB_STATUS_CONTROLLER_NOT_READY: u32 = 0x0000_0800;
const USB_COMMAND_HOST_CONTROLLER_RESET_MASK: u32 = 0x0000_0002;

pub struct XhcCapabilityRegisters {
    base_address: u64,
}

impl XhcCapabilityRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn capability_register_length(&self) -> u8 {
        get_unsigned_int_8s(unsafe { ((self.base_address + 0x00) as *const u32).read() }).0
    }

    pub fn host_controller_structural_parameters_1(&self) -> u32 {
        unsafe { ((self.base_address + 0x04) as *const u32).read() }
    }

    pub fn host_controller_cabability_parameters_1(&self) -> u32 {
        unsafe { ((self.base_address + 0x10) as *const u32).read() }
    }

    pub fn doorbell_offset(&self) -> u32 {
        unsafe { ((self.base_address + 0x14) as *const u32).read() }
    }

    pub fn runtime_register_space_offset(&self) -> u32 {
        unsafe { ((self.base_address + 0x18) as *const u32).read() }
    }
}

pub struct XhcOperationalRegisters {
    base_address: u64,
}

impl XhcOperationalRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn usb_command_host_controller_reset(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFC) + 2;
    }

    pub fn usb_command_interrupter_enable(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFB) + 4;
    }

    pub fn usb_command_run(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFE) + 1;
    }

    pub fn usb_command_stop(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (usb_command & 0xFFFF_FFFE) + 0;
    }

    pub fn set_number_of_device_slots_enabled(&self, number: u8) {
        let configure_register = self.configure_register();
        *unsafe { ((self.base_address + 0x38) as *mut u32).as_mut() }.unwrap() =
            (configure_register & 0xFFFF_FF00) + number as u32;
    }

    pub fn set_device_context_base_address_array_pointer(&self, address: u64) {
        *unsafe { ((self.base_address + 0x30) as *mut u64).as_mut() }.unwrap() =
            address & 0xFFFF_FFFF_FFFF_FFC0
    }

    pub fn set_command_ring_control_register(&self, val: u64) {
        *unsafe { ((self.base_address + 0x18) as *mut u64).as_mut() }.unwrap() = val
    }

    pub fn usb_command(&self) -> u32 {
        unsafe { ((self.base_address + 0x00) as *const u32).read() }
    }

    pub fn usb_status(&self) -> u32 {
        unsafe { ((self.base_address + 0x04) as *const u32).read() }
    }

    pub fn command_ring_control_register(&self) -> u64 {
        unsafe { ((self.base_address + 0x18) as *const u64).read() }
    }

    pub fn configure_register(&self) -> u32 {
        unsafe { ((self.base_address + 0x38) as *const u32).read() }
    }
}

pub struct XhcRuntimeRegisters {
    base_address: u64,
}

impl XhcRuntimeRegisters {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn get_interrupter_register_set(&self, index: u64) -> XhcInterrupterRegisterSet {
        XhcInterrupterRegisterSet::new(self.base_address + 0x20 + 32 * index)
    }
}

pub struct XhcInterrupterRegisterSet {
    base_address: u64,
}

impl XhcInterrupterRegisterSet {
    pub const fn new(base_address: u64) -> Self {
        Self { base_address }
    }

    pub fn interrupter_management_register(&self) -> u32 {
        unsafe { ((self.base_address + 0x00) as *const u32).read() }
    }

    pub fn event_ring_segment_table_base_address(&self) -> u64 {
        unsafe { ((self.base_address + 0x10) as *const u64).read() }
    }

    pub fn event_ring_segment_table_size(&self) -> u32 {
        unsafe { ((self.base_address + 0x08) as *const u32).read() }
    }

    pub fn event_ring_dequeue_pointer(&self) -> u64 {
        unsafe { ((self.base_address + 0x18) as *const u64).read() }
    }

    pub fn set_interrupt_pending_and_enable(&self) {
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (self.interrupter_management_register() & 0xFFFF_FFFC) + 3;
    }

    pub fn set_event_ring_segment_table_size(&self, size: u16) {
        *unsafe { ((self.base_address + 0x08) as *mut u32).as_mut() }.unwrap() =
            (size as u32) + (self.event_ring_segment_table_size() & 0xFFFF_0000);
    }

    pub fn set_event_ring_segment_table_base_address(&self, address: u64) {
        *unsafe { ((self.base_address + 0x10) as *mut u64).as_mut() }.unwrap() = (address
            & 0xFFFF_FFFF_FFFF_FFC0)
            + (self.event_ring_segment_table_base_address() & 0x3F);
    }

    pub fn set_event_ring_dequeue_pointer(&self, address: u64) {
        *unsafe { ((self.base_address + 0x18) as *mut u64).as_mut() }.unwrap() =
            (address & 0xFFFF_FFFF_FFFF_FFF0) + (self.event_ring_dequeue_pointer() & 0xF);
    }
}
