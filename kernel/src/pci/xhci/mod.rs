use common::iter_str::{IterStrFormat, Padding, Radix, ToIterStr};

use crate::{
    font::font_writer::FONT_HEIGHT,
    output_string,
    pixel_writer::pixel_color::PixelColor,
    services::Services,
    util::{get_bits_value, get_unsigned_int_8s, vector2::Vector2},
};

pub struct XhcDevice {
    capability_registers: XhcCapabilityRegisters,
    operational_registers: XhcOperationalRegisters,
}

impl XhcDevice {
    pub fn new(base_address: u64) -> Self {
        let capability_registers = XhcCapabilityRegisters::new(base_address);
        let operational_registers_offset = capability_registers.capability_register_length();
        let operational_registers =
            XhcOperationalRegisters::new(base_address + operational_registers_offset as u64);

        Self {
            capability_registers,
            operational_registers,
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
            _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, *height),
                [b"USB status host controller halted bit is zero."
                    .to_iter_str(IterStrFormat::none())]
            );
            *height += FONT_HEIGHT;
            return Err(());
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

        match services.time_services().wait_for_nano_seconds(1_000_000) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }

        'a: loop {
            if self.operational_registers.usb_command() & USB_COMMAND_HOST_CONTROLLER_RESET_MASK
                == 0
            {
                break 'a ();
            }
        }
        'a: loop {
            if self.operational_registers.usb_status() & USB_STATUS_CONTROLLER_NOT_REAY == 0 {
                break 'a ();
            }
        }

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [b"Set max device slots.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;
        let max_device_slots = get_bits_value(
            self.capability_registers
                .host_controller_structural_parameters_1(),
            0,
            7,
        ) as u8;
        self.operational_registers
            .set_number_of_device_slots_enabled(if max_device_slots < MAX_DEVICE_SLOTS_DESIRED {
                max_device_slots
            } else {
                MAX_DEVICE_SLOTS_DESIRED
            });

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
}

const USB_STATUS_HOST_CONTROLLER_HALTED_MASK: u32 = 0x0000_0001;
const USB_STATUS_CONTROLLER_NOT_REAY: u32 = 0x0000_0800;
const USB_COMMAND_HOST_CONTROLLER_RESET_MASK: u32 = 0x0000_0002;
const MAX_DEVICE_SLOTS_DESIRED: u8 = 8;

#[repr(align(64))]
struct DeviceContextBaseAddressArray {
    device_context_base_address_array: [u64; MAX_DEVICE_SLOTS_DESIRED as usize + 1],
}

// #[repr(align(64))]
// enum DeviceContextArray {
//     DeviceContextArray32([DeviceContext<32>; MAX_DEVICE_SLOTS_DESIRED as usize + 1]),
//     DeviceContextArray64([DeviceContext<64>; MAX_DEVICE_SLOTS_DESIRED as usize + 1]),
// }

// #[repr(C)]
// struct DeviceContext<const CONTEXT_SIZE: usize> {
//     slot_context: SlotContext<CONTEXT_SIZE>,
//     endpoint_contexts: [EndpointContext<CONTEXT_SIZE>; 31],
// }

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
            usb_command | USB_COMMAND_HOST_CONTROLLER_RESET_MASK;
    }

    pub fn set_number_of_device_slots_enabled(&self, number: u8) {
        let configure_register = self.configure_register();
        *unsafe { ((self.base_address + 0x00) as *mut u32).as_mut() }.unwrap() =
            (configure_register & 0xFF_FF_FF_00) + number as u32;
    }

    pub fn usb_command(&self) -> u32 {
        unsafe { ((self.base_address + 0x00) as *const u32).read() }
    }

    pub fn usb_status(&self) -> u32 {
        unsafe { ((self.base_address + 0x04) as *const u32).read() }
    }

    pub fn configure_register(&self) -> u32 {
        unsafe { ((self.base_address + 0x38) as *const u32).read() }
    }
}
