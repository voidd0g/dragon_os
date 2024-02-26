use common::{
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
    uefi::data_type::basic_type::{UnsignedInt32, UnsignedInt64, UnsignedInt8},
};

use crate::{
    font::font_writer::FONT_HEIGHT,
    output_string,
    pixel_writer::pixel_color::PixelColor,
    services::Services,
    util::{get_unsigned_int_8s, vector2::Vector2},
};

pub struct XhcDevice {
    capability_registers: XhcCapabilityRegisters,
    operational_registers: XhcOperationalRegisters,
}

impl XhcDevice {
    pub fn new(base_address: UnsignedInt64) -> Self {
        let capability_registers = XhcCapabilityRegisters::new(base_address);
        let operational_registers_offset = capability_registers.capability_register_length();
        let operational_registers = XhcOperationalRegisters::new(
            base_address + operational_registers_offset as UnsignedInt64,
        );

        Self {
            capability_registers,
            operational_registers,
        }
    }

    pub fn initialize(&self, services: &Services, height: &mut UnsignedInt32) -> Result<(), ()> {
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"USB status: ".to_iter_str(IterStrFormat::none()),
                self.operational_registers
                    .usb_status()
                    .to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    )),
                b"; USB command: ".to_iter_str(IterStrFormat::none()),
                self.operational_registers
                    .usb_command()
                    .to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    )),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

        if self.operational_registers.usb_status() & USB_STATUS_HOST_CONTROLLER_HALTED_MASK == 0 {
            _ =
                output_string!(
                    services,
                    PixelColor::new(128, 0, 0),
                    Vector2::new(0, *height),
                    [b"USB status host controller halted is zero."
                        .to_iter_str(IterStrFormat::none())]
                );
            *height += FONT_HEIGHT;
            return Err(());
        }

        self.operational_registers
            .usb_command_host_controller_reset();

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, *height),
            [
                b"USB status: ".to_iter_str(IterStrFormat::none()),
                self.operational_registers
                    .usb_status()
                    .to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    )),
                b"; USB command: ".to_iter_str(IterStrFormat::none()),
                self.operational_registers
                    .usb_command()
                    .to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    )),
            ]
        ) {
            Ok(()) => (),
            Err(()) => return Err(()),
        }
        *height += FONT_HEIGHT;

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

        Ok(())
    }
}

const USB_STATUS_HOST_CONTROLLER_HALTED_MASK: UnsignedInt32 = 0x0000_0001;
const USB_STATUS_CONTROLLER_NOT_REAY: UnsignedInt32 = 0x0000_0800;
const USB_COMMAND_HOST_CONTROLLER_RESET_MASK: UnsignedInt32 = 0x0000_0002;

pub struct XhcCapabilityRegisters {
    base_address: UnsignedInt64,
}

impl XhcCapabilityRegisters {
    pub const fn new(base_address: UnsignedInt64) -> Self {
        Self { base_address }
    }

    pub fn capability_register_length(&self) -> UnsignedInt8 {
        get_unsigned_int_8s(unsafe { ((self.base_address + 0x00) as *const UnsignedInt32).read() })
            .0
    }

    pub fn runtime_register_space_offset(&self) -> UnsignedInt32 {
        unsafe { ((self.base_address + 0x18) as *const UnsignedInt32).read() }
    }

    pub fn doorbell_offset(&self) -> UnsignedInt32 {
        unsafe { ((self.base_address + 0x14) as *const UnsignedInt32).read() }
    }
}

pub struct XhcOperationalRegisters {
    base_address: UnsignedInt64,
}

impl XhcOperationalRegisters {
    pub const fn new(base_address: UnsignedInt64) -> Self {
        Self { base_address }
    }

    pub fn usb_command_host_controller_reset(&self) {
        let usb_command = self.usb_command();
        *unsafe { ((self.base_address + 0x00) as *mut UnsignedInt32).as_mut() }.unwrap() =
            usb_command | USB_COMMAND_HOST_CONTROLLER_RESET_MASK;
    }

    pub fn usb_command(&self) -> UnsignedInt32 {
        unsafe { ((self.base_address + 0x00) as *const UnsignedInt32).read() }
    }

    pub fn usb_status(&self) -> UnsignedInt32 {
        unsafe { ((self.base_address + 0x04) as *const UnsignedInt32).read() }
    }
}
