#![no_std]
#![no_main]

mod font;
mod pci;
mod pixel_writer;
mod pointer;
mod services;
mod util;

use core::{arch::asm, panic::PanicInfo};

use common::{
    argument::Argument,
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
};
use font::font_writer::FONT_HEIGHT;

use crate::{
    pci::{xhci::XhcDevice, BusScanner},
    pixel_writer::{draw_rect::DrawRect, pixel_color::PixelColor},
    pointer::PointerWriter,
    services::Services,
    util::vector2::Vector2,
};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub extern "sysv64" fn kernel_main(arg: *const Argument) -> ! {
    let arg = unsafe { arg.as_ref() }.unwrap();

    let frame_buffer_config = arg.frame_buffer_config();
    let runtime_services = arg.runtime_services();

    let services = Services::new(frame_buffer_config, runtime_services);

    let mut height = 0;

    match services.draw_services().put_pixels(DrawRect::new(
        PixelColor::new(0, 255, 128),
        Vector2::new(0, 0),
        Vector2::new(
            frame_buffer_config.horizontal_resolution(),
            frame_buffer_config.vertical_resolution(),
        ),
    )) {
        Ok(()) => (),
        Err(()) => {
            _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                [b"Failed to draw background rect.".to_iter_str(IterStrFormat::none())]
            );
            end()
        }
    }

    let mut bus_scanner = BusScanner::new();
    match bus_scanner.scan_all_devices() {
        Ok(()) => (),
        Err(()) => {
            _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                [b"Failed to scan devices from bus.".to_iter_str(IterStrFormat::none())]
            );
            end()
        }
    }
    match output_string!(
        services,
        PixelColor::new(128, 0, 0),
        Vector2::new(0, height),
        [
            bus_scanner
                .devices_found()
                .len()
                .to_iter_str(IterStrFormat::none()),
            b" devices found.".to_iter_str(IterStrFormat::none()),
        ]
    ) {
        Ok(()) => (),
        Err(()) => end(),
    };
    height += FONT_HEIGHT;

    for device in bus_scanner.devices_found() {
        let class_codes = device.class_codes();

        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, height),
            [
                device.bus().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                device.device().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                device.function().to_iter_str(IterStrFormat::none()),
                b": vendor_id ".to_iter_str(IterStrFormat::none()),
                device.vendor_id().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    Some(true),
                    Some(Padding::new(b'0', 4))
                )),
                b", class_codes ".to_iter_str(IterStrFormat::none()),
                class_codes.base_class().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    Some(true),
                    Some(Padding::new(b'0', 2))
                )),
                b"-".to_iter_str(IterStrFormat::none()),
                class_codes.sub_class().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    Some(true),
                    Some(Padding::new(b'0', 2))
                )),
                b"-".to_iter_str(IterStrFormat::none()),
                class_codes.interface().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    Some(true),
                    Some(Padding::new(b'0', 2))
                )),
                b", header_type ".to_iter_str(IterStrFormat::none()),
                device.header_type().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    Some(true),
                    Some(Padding::new(b'0', 2))
                )),
            ]
        ) {
            Ok(()) => (),
            Err(()) => end(),
        };
        height += FONT_HEIGHT;
    }

    match output_string!(
        services,
        PixelColor::new(128, 0, 0),
        Vector2::new(0, height),
        [b"End of the device list.".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(()) => (),
        Err(()) => end(),
    };
    height += FONT_HEIGHT;

    const XHCI_BASE_CLASS: u8 = 0x0C;
    const XHCI_SUB_CLASS: u8 = 0x03;
    const XHCI_INTERFACE: u8 = 0x30;
    const INTEL_VENDOR_ID: u16 = 0x8086;

    let mut xhci_found = None;
    for device in bus_scanner.devices_found() {
        let class_codes = device.class_codes();
        if class_codes.base_class() == XHCI_BASE_CLASS
            && class_codes.sub_class() == XHCI_SUB_CLASS
            && class_codes.interface() == XHCI_INTERFACE
        {
            xhci_found = Some(device);
            if device.vendor_id() == INTEL_VENDOR_ID {
                break;
            }
        }
    }

    let xhci_found = match xhci_found {
        Some(xhci_found) => xhci_found,
        None => {
            let _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                [b"No xHC device was found.".to_iter_str(IterStrFormat::none())]
            );
            end()
        }
    };

    match output_string!(
        services,
        PixelColor::new(128, 0, 0),
        Vector2::new(0, height),
        [
            xhci_found.bus().to_iter_str(IterStrFormat::none()),
            b".".to_iter_str(IterStrFormat::none()),
            xhci_found.device().to_iter_str(IterStrFormat::none()),
            b".".to_iter_str(IterStrFormat::none()),
            xhci_found.function().to_iter_str(IterStrFormat::none()),
            b" was xHC device. ".to_iter_str(IterStrFormat::none()),
        ]
    ) {
        Ok(()) => (),
        Err(()) => end(),
    };
    height += FONT_HEIGHT;

    if xhci_found.vendor_id() == INTEL_VENDOR_ID {
        const EHCI_BASE_CLASS: u8 = 0x0C;
        const EHCI_SUB_CLASS: u8 = 0x03;
        const EHCI_INTERFACE: u8 = 0x20;
        for device in bus_scanner.devices_found() {
            let class_codes = device.class_codes();
            if class_codes.base_class() == EHCI_BASE_CLASS
                && class_codes.sub_class() == EHCI_SUB_CLASS
                && class_codes.interface() == EHCI_INTERFACE
                && device.vendor_id() == INTEL_VENDOR_ID
            {
                xhci_found.enable_super_speed();
                xhci_found.switch_ehci_to_xhci();

                break;
            }
        }
    }

    let xhci_mmio_base = xhci_found.base_address_register0();

    match output_string!(
        services,
        PixelColor::new(128, 0, 0),
        Vector2::new(0, height),
        [
            b"xHCI MMIO base address is ".to_iter_str(IterStrFormat::none()),
            xhci_mmio_base.to_iter_str(IterStrFormat::new(
                Some(Radix::Hexadecimal),
                Some(true),
                Some(Padding::new(b'0', 16))
            )),
        ]
    ) {
        Ok(()) => (),
        Err(()) => end(),
    };
    height += FONT_HEIGHT;

    let xhc_device = XhcDevice::new(xhci_mmio_base);

    match xhc_device.initialize(&services, &mut height) {
        Ok(()) => (),
        Err(()) => {
            _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                [b"Failed to initialize xhc device.".to_iter_str(IterStrFormat::none())]
            );
            end()
        }
    }

    match output_string!(
        services,
        PixelColor::new(128, 0, 0),
        Vector2::new(0, height),
        [b"Successfully initialized xHC device.".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(()) => (),
        Err(()) => end(),
    };
    height += FONT_HEIGHT;

    match services
        .draw_services()
        .put_pixels(PointerWriter::new(Vector2::new(300, 300)))
    {
        Ok(()) => (),
        Err(()) => {
            _ = output_string!(
                services,
                PixelColor::new(128, 0, 0),
                Vector2::new(0, height),
                [b"Failed to draw pointer.".to_iter_str(IterStrFormat::none())]
            );
            end()
        }
    }

    end()
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
