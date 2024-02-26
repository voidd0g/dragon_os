pub mod xhci;

use core::arch::asm;

use crate::util::{get_unsigned_int_16s, get_unsigned_int_8s};

fn make_pci_config_address(bus: u8, device: u8, function: u8, register_address: u8) -> u32 {
    const ENABLE_BIT_SHL: u32 = 31;
    const ENABLE_BIT_MASK: u32 = 0x8000_0000;
    const BUS_SHL: u32 = 16;
    const BUS_MASK: u32 = 0x00FF_0000;
    const DEVICE_SHL: u32 = 11;
    const DEVICE_MASK: u32 = 0x0000_F800;
    const FUNCTION_SHL: u32 = 8;
    const FUNCTION_MASK: u32 = 0x0000_0700;
    const REGISTER_ADDRESS_SHL: u32 = 0;
    const REGISTER_ADDRESS_MASK: u32 = 0x0000_00FF;
    ((1 << ENABLE_BIT_SHL) & ENABLE_BIT_MASK)
        | (((bus as u32) << BUS_SHL) & BUS_MASK)
        | (((device as u32) << DEVICE_SHL) & DEVICE_MASK)
        | (((function as u32) << FUNCTION_SHL) & FUNCTION_MASK)
        | (((register_address as u32) << REGISTER_ADDRESS_SHL) & REGISTER_ADDRESS_MASK)
}

fn io_in(address: u16) -> u32 {
    let ret;
    unsafe {
        asm!("in eax, dx", out("eax") ret, in("dx") address);
    }
    ret
}

fn io_out(address: u16, data: u32) {
    unsafe {
        asm!("out dx, eax", in("dx") address, in("eax") data);
    }
}

const CONFIG_ADDRESS_ADDRESS: u16 = 0x0CF8;
const CONFIG_DATA_ADDRESS: u16 = 0x0CFC;

fn write_config_address(address: u32) {
    io_out(CONFIG_ADDRESS_ADDRESS, address)
}
fn write_config_data(data: u32) {
    io_out(CONFIG_DATA_ADDRESS, data)
}
fn read_config_data() -> u32 {
    io_in(CONFIG_DATA_ADDRESS)
}

const INVALID_VENDOR_ID: u16 = 0xFFFF;

fn read_vendor_id(bus: u8, device: u8, function: u8) -> u16 {
    write_config_address(make_pci_config_address(bus, device, function, 0x00));
    get_unsigned_int_16s(read_config_data()).0
}
fn read_header_type(bus: u8, device: u8, function: u8) -> u8 {
    write_config_address(make_pci_config_address(bus, device, function, 0x0C));
    get_unsigned_int_8s(read_config_data()).2
}
fn read_class_code(bus: u8, device: u8, function: u8) -> u32 {
    write_config_address(make_pci_config_address(bus, device, function, 0x08));
    read_config_data()
}
fn read_bus_numbers(bus: u8, device: u8, function: u8) -> u32 {
    write_config_address(make_pci_config_address(bus, device, function, 0x18));
    read_config_data()
}
fn read_base_address_register0(bus: u8, device: u8, function: u8) -> u64 {
    write_config_address(make_pci_config_address(bus, device, function, 0x10));
    let lo = read_config_data();

    if lo & 0x0000_0006 == 0 {
        (lo as u64) & 0xFFFF_FFFF_FFFF_FFF0
    } else {
        write_config_address(make_pci_config_address(bus, device, function, 0x14));
        let hi = read_config_data();
        (lo as u64 + ((hi as u64) << 32)) & 0xFFFF_FFFF_FFFF_FFF0
    }
}
fn read_xusb2_port_routing_mask(bus: u8, device: u8, function: u8) -> u32 {
    write_config_address(make_pci_config_address(bus, device, function, 0xD4));
    read_config_data()
}
fn read_usb3_port_routing_mask(bus: u8, device: u8, function: u8) -> u32 {
    write_config_address(make_pci_config_address(bus, device, function, 0xDC));
    read_config_data()
}

fn write_xusb2_port_routing(bus: u8, device: u8, function: u8, value: u32) {
    write_config_address(make_pci_config_address(bus, device, function, 0xD0));
    write_config_data(value)
}
fn write_usb3_port_super_speed_enable(bus: u8, device: u8, function: u8, value: u32) {
    write_config_address(make_pci_config_address(bus, device, function, 0xD8));
    write_config_data(value)
}

fn is_single_function_device(header_type: u8) -> bool {
    (header_type & 0x80) == 0
}

pub struct BusScanner {
    devices_found: [PciDevice; 32],
    devices_count: usize,
}

const FUNCTION_COUNT: u8 = 8;
const DEVICE_COUNT: u8 = 32;

impl BusScanner {
    pub const fn new() -> Self {
        Self {
            devices_found: [PciDevice::new(0, 0, 0, 0, PciDevieClassCodes::new(0, 0, 0, 0), 0); 32],
            devices_count: 0,
        }
    }

    pub fn devices_found(&self) -> &[PciDevice] {
        &self.devices_found[..if self.devices_count < self.devices_found.len() {
            self.devices_count
        } else {
            self.devices_found.len()
        }]
    }

    pub fn scan_all_devices(&mut self) -> Result<(), ()> {
        self.devices_count = 0;

        let host_bridge_header_type = read_header_type(0, 0, 0);
        if is_single_function_device(host_bridge_header_type) {
            let _ = match self.scan_bus(0) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
        } else {
            for function in 1..FUNCTION_COUNT {
                let vendor_id = read_vendor_id(0, 0, function);
                if vendor_id != INVALID_VENDOR_ID {
                    let _ = match self.scan_bus(function) {
                        Ok(res) => res,
                        Err(v) => return Err(v),
                    };
                }
            }
            return Err(());
        }
        Ok(())
    }

    fn scan_bus(&mut self, bus: u8) -> Result<(), ()> {
        for device in 0..DEVICE_COUNT {
            let vendor_id = read_vendor_id(bus, device, 0);
            if vendor_id != INVALID_VENDOR_ID {
                let _ = match self.scan_device(bus, device) {
                    Ok(res) => res,
                    Err(v) => return Err(v),
                };
            }
        }
        Ok(())
    }

    fn scan_device(&mut self, bus: u8, device: u8) -> Result<(), ()> {
        let _ = match self.scan_function(bus, device, 0) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };

        let primary_function_header_type = read_header_type(bus, device, 0);
        if !is_single_function_device(primary_function_header_type) {
            for function in 1..FUNCTION_COUNT {
                let vendor_id = read_vendor_id(bus, device, function);
                if vendor_id != INVALID_VENDOR_ID {
                    let _ = match self.scan_function(bus, device, function) {
                        Ok(res) => res,
                        Err(v) => return Err(v),
                    };
                }
            }
        }
        Ok(())
    }

    fn scan_function(&mut self, bus: u8, device: u8, function: u8) -> Result<(), ()> {
        let header_type = read_header_type(bus, device, function);
        let _ = match self.add_device(bus, device, function, header_type) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };

        let (_, _, sub_class, base_class) =
            get_unsigned_int_8s(read_class_code(bus, device, function));

        if base_class == 0x06 && sub_class == 0x04 {
            let secondary_bus = get_unsigned_int_8s(read_bus_numbers(bus, device, function)).1;
            let _ = match self.scan_bus(secondary_bus) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
        }

        Ok(())
    }

    fn add_device(&mut self, bus: u8, device: u8, function: u8, header_type: u8) -> Result<(), ()> {
        match self.devices_found.get_mut(self.devices_count) {
            Some(found) => {
                let vendor_id = read_vendor_id(bus, device, function);
                let (revision_id, interface, sub_class, base_class) =
                    get_unsigned_int_8s(read_class_code(bus, device, function));
                *found = PciDevice::new(
                    bus,
                    device,
                    function,
                    vendor_id,
                    PciDevieClassCodes::new(base_class, sub_class, interface, revision_id),
                    header_type,
                );
                self.devices_count += 1;
            }
            None => return Err(()),
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct PciDevice {
    bus: u8,
    device: u8,
    function: u8,
    vendor_id: u16,
    class_codes: PciDevieClassCodes,
    header_type: u8,
}

impl PciDevice {
    pub const fn new(
        bus: u8,
        device: u8,
        function: u8,
        vendor_id: u16,
        class_codes: PciDevieClassCodes,
        header_type: u8,
    ) -> Self {
        Self {
            bus,
            device,
            function,
            vendor_id,
            class_codes,
            header_type,
        }
    }

    pub fn bus(&self) -> u8 {
        self.bus
    }
    pub fn device(&self) -> u8 {
        self.device
    }
    pub fn function(&self) -> u8 {
        self.function
    }
    pub fn vendor_id(&self) -> u16 {
        self.vendor_id
    }
    pub fn class_codes(&self) -> PciDevieClassCodes {
        self.class_codes
    }
    pub fn header_type(&self) -> u8 {
        self.header_type
    }

    pub fn base_address_register0(&self) -> u64 {
        read_base_address_register0(self.bus, self.device, self.function)
    }

    pub fn enable_super_speed(&self) {
        write_usb3_port_super_speed_enable(
            self.bus,
            self.device,
            self.function,
            read_usb3_port_routing_mask(self.bus, self.device, self.function),
        )
    }
    pub fn switch_ehci_to_xhci(&self) {
        write_xusb2_port_routing(
            self.bus,
            self.device,
            self.function,
            read_xusb2_port_routing_mask(self.bus, self.device, self.function),
        )
    }
}

#[derive(Clone, Copy)]
pub struct PciDevieClassCodes {
    base_class: u8,
    sub_class: u8,
    interface: u8,
    revision_id: u8,
}

impl PciDevieClassCodes {
    pub const fn new(base_class: u8, sub_class: u8, interface: u8, revision_id: u8) -> Self {
        Self {
            base_class,
            sub_class,
            interface,
            revision_id,
        }
    }

    pub fn base_class(&self) -> u8 {
        self.base_class
    }
    pub fn sub_class(&self) -> u8 {
        self.sub_class
    }
    pub fn interface(&self) -> u8 {
        self.interface
    }
}
