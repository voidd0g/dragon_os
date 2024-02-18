use core::arch::asm;

use common::uefi::data_type::basic_type::{
    UnsignedInt16, UnsignedInt32, UnsignedInt8, UnsignedIntNative,
};

use crate::util::{get_unsigned_int_16s, get_unsigned_int_8s};

fn make_pci_config_address(
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
    register_address: UnsignedInt8,
) -> UnsignedInt32 {
    const ENABLE_BIT_SHL: UnsignedInt32 = 1;
    const ENABLE_BIT_MASK: UnsignedInt32 = 0x8000_0000;
    const BUS_SHL: UnsignedInt32 = 16;
    const BUS_MASK: UnsignedInt32 = 0x00FF_0000;
    const DEVICE_SHL: UnsignedInt32 = 11;
    const DEVICE_MASK: UnsignedInt32 = 0x0000_F800;
    const FUNCTION_SHL: UnsignedInt32 = 8;
    const FUNCTION_MASK: UnsignedInt32 = 0x0000_0700;
    const REGISTER_ADDRESS_SHL: UnsignedInt32 = 0;
    const REGISTER_ADDRESS_MASK: UnsignedInt32 = 0x0000_00FF;
    ((1 << ENABLE_BIT_SHL) & ENABLE_BIT_MASK)
        | (((bus as UnsignedInt32) << BUS_SHL) & BUS_MASK)
        | (((device as UnsignedInt32) << DEVICE_SHL) & DEVICE_MASK)
        | (((function as UnsignedInt32) << FUNCTION_SHL) & FUNCTION_MASK)
        | (((register_address as UnsignedInt32) << REGISTER_ADDRESS_SHL) & REGISTER_ADDRESS_MASK)
}

fn io_in(address: UnsignedInt16) -> UnsignedInt32 {
    let ret;
    unsafe {
        asm!("in eax, dx", out("eax") ret, in("dx") address);
    }
    ret
}

fn io_out(address: UnsignedInt16, data: UnsignedInt32) {
    unsafe {
        asm!("out dx, eax", in("dx") address, in("eax") data);
    }
}

const CONFIG_ADDRESS_ADDRESS: UnsignedInt16 = 0x0CF8;
const CONFIG_DATA_ADDRESS: UnsignedInt16 = 0x0CFC;

fn write_config_address(address: UnsignedInt32) {
    io_out(CONFIG_ADDRESS_ADDRESS, address)
}
fn write_config_data(address: UnsignedInt32) {
    io_out(CONFIG_DATA_ADDRESS, address)
}
fn read_config_data() -> UnsignedInt32 {
    io_in(CONFIG_DATA_ADDRESS)
}

const INVALID_VENDOR_ID: UnsignedInt16 = 0xFFFF;

fn read_vendor_id(
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
) -> UnsignedInt16 {
    write_config_address(make_pci_config_address(bus, device, function, 0x00));
    get_unsigned_int_16s(read_config_data()).0
}
fn read_header_type(
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
) -> UnsignedInt8 {
    write_config_address(make_pci_config_address(bus, device, function, 0x0C));
    get_unsigned_int_8s(read_config_data()).2
}
fn read_class_code(
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
) -> UnsignedInt32 {
    write_config_address(make_pci_config_address(bus, device, function, 0x08));
    read_config_data()
}
fn read_bus_numbers(
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
) -> UnsignedInt32 {
    write_config_address(make_pci_config_address(bus, device, function, 0x18));
    read_config_data()
}
fn is_single_function_device(header_type: UnsignedInt8) -> bool {
    header_type & 0x80 == 0
}

pub struct BusScanner {
    devices_found: [PciDevice; 32],
    devices_count: UnsignedIntNative,
}

const FUNCTION_COUNT: UnsignedInt8 = 8;
const DEVICE_COUNT: UnsignedInt8 = 32;

impl BusScanner {
    pub const fn new() -> Self {
        Self {
            devices_found: [PciDevice::new(0, 0, 0, 0); 32],
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
        }
        Ok(())
    }

    fn scan_bus(&mut self, bus: UnsignedInt8) -> Result<(), ()> {
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

    fn scan_device(&mut self, bus: UnsignedInt8, device: UnsignedInt8) -> Result<(), ()> {
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

    fn scan_function(
        &mut self,
        bus: UnsignedInt8,
        device: UnsignedInt8,
        function: UnsignedInt8,
    ) -> Result<(), ()> {
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

    fn add_device(
        &mut self,
        bus: UnsignedInt8,
        device: UnsignedInt8,
        function: UnsignedInt8,
        header_type: UnsignedInt8,
    ) -> Result<(), ()> {
        match self.devices_found.get_mut(self.devices_count) {
            Some(found) => {
                *found = PciDevice::new(bus, device, function, header_type);
                self.devices_count += 1;
            }
            None => return Err(()),
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
pub struct PciDevice {
    bus: UnsignedInt8,
    device: UnsignedInt8,
    function: UnsignedInt8,
    header_type: UnsignedInt8,
}

impl PciDevice {
    pub const fn new(
        bus: UnsignedInt8,
        device: UnsignedInt8,
        function: UnsignedInt8,
        header_type: UnsignedInt8,
    ) -> Self {
        Self {
            bus,
            device,
            function,
            header_type,
        }
    }
}
