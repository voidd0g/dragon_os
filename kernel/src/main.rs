#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(generic_const_exprs)]
#![feature(abi_x86_interrupt)]

mod font;
mod interrupt;
mod memory_manager;
mod paging;
mod pci;
mod pixel_writer;
mod pointer;
mod segment;
mod services;
mod util;

use core::{
    arch::{asm, global_asm},
    panic::PanicInfo,
};

use common::{
    argument::Argument,
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
};
use font::font_writer::FONT_HEIGHT;
use interrupt::pop_interrupt_queue;

use crate::{
    interrupt::{
        interrupt_vector::{
            INTERRUPT_VECTOR_XHCI_SLOT_0, INTERRUPT_VECTOR_XHCI_SLOT_1,
            INTERRUPT_VECTOR_XHCI_SLOT_2, INTERRUPT_VECTOR_XHCI_SLOT_3,
            XHCI_HOST_CONTROLLER_MAX_COUNT,
        },
        setup_interrupt_descriptor_table, InterruptMessage,
    },
    memory_manager::BitmapMemoryManager,
    paging::setup_identity_page_table_2m,
    pci::{
        local_apic::local_apic_id, msi_delivery_mode::MSI_DELIVERY_MODE_FIXED, xhci::XhcDevice,
        BusScanner, PciDevice,
    },
    pixel_writer::{draw_rect::DrawRect, pixel_color::PixelColor},
    pointer::PointerWriter,
    segment::setup_segments,
    services::Services,
    util::vector2::Vector2,
};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

const KERNEL_MAIN_STACK_ALIGN: usize = 16;
const KERNEL_MAIN_STACK_SIZE: usize = 0x1000000;

global_asm!(
    r#"
.extern kernel_main_core

.section .bss
.align {KERNEL_MAIN_STACK_ALIGN}
KERNEL_MAIN_STACK:
    .space {KERNEL_MAIN_STACK_SIZE}

.section .text
.global kernel_main
kernel_main:
    lea rsp, KERNEL_MAIN_STACK[rip]
    add rsp, {KERNEL_MAIN_STACK_SIZE}
    call kernel_main_core
.fin:
    hlt
    jmp .fin
"#,
    KERNEL_MAIN_STACK_ALIGN = const { KERNEL_MAIN_STACK_ALIGN },
    KERNEL_MAIN_STACK_SIZE = const { KERNEL_MAIN_STACK_SIZE }
);

#[no_mangle]
pub extern "sysv64" fn kernel_main_core(arg: *const Argument) -> ! {
    let arg = unsafe { arg.as_ref() }.unwrap();

    let frame_buffer_config = arg.frame_buffer_config();
    let runtime_services = arg.runtime_services();
    let memory_map = arg.memory_map();
    let services = Services::new(frame_buffer_config, runtime_services);
    let draw_service = services.draw_services();

    let memory_manager = BitmapMemoryManager::new(memory_map);

    setup_segments();

    setup_identity_page_table_2m();

    let mut height = 0;

    match draw_service.put_pixels(DrawRect::new(
        PixelColor::new(0, 255, 128),
        Vector2::new(0, 0),
        Vector2::new(
            draw_service.horizontal_resolution(),
            draw_service.vertical_resolution(),
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
    height %= frame_buffer_config.vertical_resolution();

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
                b".".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => end(),
        };
        height += FONT_HEIGHT;
        height %= frame_buffer_config.vertical_resolution();
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
    height %= frame_buffer_config.vertical_resolution();

    const XHCI_BASE_CLASS: u8 = 0x0C;
    const XHCI_SUB_CLASS: u8 = 0x03;
    const XHCI_INTERFACE: u8 = 0x30;
    const INTEL_VENDOR_ID: u16 = 0x8086;

    const XHCIS_FOUND_RESET_VALUE: Option<&PciDevice> = None;
    let mut xhcis_found = [XHCIS_FOUND_RESET_VALUE; XHCI_HOST_CONTROLLER_MAX_COUNT];
    let mut xhci_found_count = 0;

    for device in bus_scanner.devices_found() {
        let class_codes = device.class_codes();
        if class_codes.base_class() == XHCI_BASE_CLASS
            && class_codes.sub_class() == XHCI_SUB_CLASS
            && class_codes.interface() == XHCI_INTERFACE
        {
            if xhci_found_count < XHCI_HOST_CONTROLLER_MAX_COUNT {
                xhcis_found[xhci_found_count] = Some(device);
                xhci_found_count += 1;
            }
        }
    }

    if xhci_found_count == 0 {
        let _ = output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, height),
            [b"No xHC device was found.".to_iter_str(IterStrFormat::none())]
        );
        end()
    }

    setup_interrupt_descriptor_table();

    const XHC_DEVICES_RESET_VALUE: Option<XhcDevice> = None;
    let mut xhc_devices = [XHC_DEVICES_RESET_VALUE; XHCI_HOST_CONTROLLER_MAX_COUNT];

    for i in 0..xhci_found_count {
        let xhci_found = xhcis_found[i].unwrap();
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, height),
            [
                b"Processing xHC device \"".to_iter_str(IterStrFormat::none()),
                xhci_found.bus().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                xhci_found.device().to_iter_str(IterStrFormat::none()),
                b".".to_iter_str(IterStrFormat::none()),
                xhci_found.function().to_iter_str(IterStrFormat::none()),
                b"\".".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => end(),
        };
        height += FONT_HEIGHT;
        height %= frame_buffer_config.vertical_resolution();

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
                b".".to_iter_str(IterStrFormat::none()),
            ]
        ) {
            Ok(()) => (),
            Err(()) => end(),
        };
        height += FONT_HEIGHT;
        height %= frame_buffer_config.vertical_resolution();

        let bsp_local_apic_id = local_apic_id();
        match xhci_found.configure_msi_fixed_destination(
            bsp_local_apic_id,
            true,
            true,
            MSI_DELIVERY_MODE_FIXED,
            match i {
                0 => INTERRUPT_VECTOR_XHCI_SLOT_0,
                1 => INTERRUPT_VECTOR_XHCI_SLOT_1,
                2 => INTERRUPT_VECTOR_XHCI_SLOT_2,
                3 => INTERRUPT_VECTOR_XHCI_SLOT_3,
                _ => INTERRUPT_VECTOR_XHCI_SLOT_0,
            },
            0,
        ) {
            Ok(()) => (),
            Err(()) => {
                _ = output_string!(
                    services,
                    PixelColor::new(128, 0, 0),
                    Vector2::new(0, height),
                    [b"Failed to set bsp local apic id to msi config."
                        .to_iter_str(IterStrFormat::none())]
                );
                end()
            }
        }

        let xhc_device = XhcDevice::new(xhci_mmio_base);
        xhc_devices[i] = Some(xhc_device);
        let xhc_device = xhc_devices[i].as_mut().unwrap();

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

        match xhc_device.run(&services, &mut height) {
            Ok(()) => (),
            Err(()) => {
                _ = output_string!(
                    services,
                    PixelColor::new(128, 0, 0),
                    Vector2::new(0, height),
                    [b"Failed to run usb device.".to_iter_str(IterStrFormat::none())]
                );
                end()
            }
        }

        match xhc_device.reset_ports(&services, &mut height) {
            Ok(()) => (),
            Err(()) => {
                _ = output_string!(
                    services,
                    PixelColor::new(128, 0, 0),
                    Vector2::new(0, height),
                    [b"Failed to reset usb ports.".to_iter_str(IterStrFormat::none())]
                );
                end()
            }
        }
    }

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

    match draw_service.put_pixels(DrawRect::new(
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
    height = 0;
    loop {
        unsafe {
            asm!("cli");
        }
        match output_string!(
            services,
            PixelColor::new(128, 0, 0),
            Vector2::new(0, height),
            [b"Check for interrupt queue.".to_iter_str(IterStrFormat::none()),]
        ) {
            Ok(()) => (),
            Err(()) => end(),
        };
        height += FONT_HEIGHT;
        height %= frame_buffer_config.vertical_resolution();
        let popped = pop_interrupt_queue();
        match popped {
            Some(v) => {
                unsafe {
                    asm!("sti");
                }
                match v {
                    InterruptMessage::XhciInterrupt(index) => {
                        match output_string!(
                            services,
                            PixelColor::new(128, 0, 0),
                            Vector2::new(0, height),
                            [
                                b"xHCI interrupt index ".to_iter_str(IterStrFormat::none()),
                                index.to_iter_str(IterStrFormat::none()),
                                b" caught.".to_iter_str(IterStrFormat::none()),
                            ]
                        ) {
                            Ok(()) => (),
                            Err(()) => end(),
                        };
                        height += FONT_HEIGHT;
                        height %= frame_buffer_config.vertical_resolution();
                        match xhc_devices[index]
                            .as_mut()
                            .unwrap()
                            .process_events(&services, &mut height)
                        {
                            Ok(()) => (),
                            Err(()) => {
                                _ = output_string!(
                                    services,
                                    PixelColor::new(128, 0, 0),
                                    Vector2::new(0, height),
                                    [b"Failed to process event."
                                        .to_iter_str(IterStrFormat::none())]
                                );
                                end()
                            }
                        }
                    }
                }
            }
            None => unsafe {
                asm!("sti", "hlt");
            },
        }
    }
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
