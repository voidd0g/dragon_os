use core::mem::size_of;

#[repr(align(0x1000))]
pub struct DeviceContextBaseAddressArray<const ARRAY_SIZE: usize>
where
    [(); ARRAY_SIZE + 1]:,
{
    dcbaa: [u64; ARRAY_SIZE + 1],
}
impl<const ARRAY_SIZE: usize> DeviceContextBaseAddressArray<ARRAY_SIZE>
where
    [(); ARRAY_SIZE + 1]:,
{
    pub const fn new() -> Self {
        Self {
            dcbaa: [0; ARRAY_SIZE + 1],
        }
    }

    pub fn pointer(&self) -> *const u64 {
        self.dcbaa.as_ptr()
    }

    pub fn register_pointer(&mut self, index: usize, pointer: u64) {
        self.dcbaa[index] = pointer;
    }
}

#[repr(align(0x1000))]
pub struct DeviceContexts<const CONTEXT_COUNT: usize>
where
    [(); CONTEXT_COUNT * 0x800]:,
{
    contexts: [u8; CONTEXT_COUNT * 0x800],
}

impl<'a, const CONTEXT_COUNT: usize> DeviceContexts<CONTEXT_COUNT>
where
    [(); CONTEXT_COUNT * 0x800]:,
{
    pub const fn new() -> Self {
        Self {
            contexts: [0; CONTEXT_COUNT * 0x800],
        }
    }

    pub fn as_ptr_32(&self, index: usize) -> *const DeviceContext32 {
        (self.contexts.as_ptr() as usize + index * size_of::<DeviceContext32>())
            as *const DeviceContext32
    }

    pub fn as_ptr_64(&self, index: usize) -> *const DeviceContext64 {
        (self.contexts.as_ptr() as usize + index * size_of::<DeviceContext64>())
            as *const DeviceContext64
    }

    pub fn as_mut_32(&'a mut self, index: usize) -> &'a mut DeviceContext32 {
        unsafe {
            ((self.contexts.as_ptr() as usize + index * size_of::<DeviceContext32>())
                as *mut DeviceContext32)
                .as_mut()
        }
        .unwrap()
    }

    pub fn as_mut_64(&'a mut self, index: usize) -> &'a mut DeviceContext64 {
        unsafe {
            ((self.contexts.as_ptr() as usize + index * size_of::<DeviceContext64>())
                as *mut DeviceContext64)
                .as_mut()
        }
        .unwrap()
    }
}

#[repr(align(0x1000))]
pub struct InputContexts<const CONTEXT_COUNT: usize>
where
    [(); CONTEXT_COUNT * 0x1000]:,
{
    contexts: [u8; CONTEXT_COUNT * 0x1000],
}

impl<'a, const CONTEXT_COUNT: usize> InputContexts<CONTEXT_COUNT>
where
    [(); CONTEXT_COUNT * 0x1000]:,
{
    pub const fn new() -> Self {
        Self {
            contexts: [0; CONTEXT_COUNT * 0x1000],
        }
    }

    pub fn as_ptr_32(&self, index: usize) -> *const InputContext32 {
        (self.contexts.as_ptr() as usize + index * size_of::<InputContext32>())
            as *const InputContext32
    }

    pub fn as_ptr_64(&self, index: usize) -> *const InputContext64 {
        (self.contexts.as_ptr() as usize + index * size_of::<InputContext64>())
            as *const InputContext64
    }

    pub fn as_mut_32(&'a mut self, index: usize) -> &'a mut InputContext32 {
        unsafe {
            ((self.contexts.as_ptr() as usize + index * size_of::<InputContext32>())
                as *mut InputContext32)
                .as_mut()
        }
        .unwrap()
    }

    pub fn as_mut_64(&'a mut self, index: usize) -> &'a mut InputContext64 {
        unsafe {
            ((self.contexts.as_ptr() as usize + index * size_of::<InputContext64>())
                as *mut InputContext64)
                .as_mut()
        }
        .unwrap()
    }
}

#[repr(C)]
pub struct InputContext32 {
    input_control_context: InputControlContext32,
    slot_context: SlotContext32,
    endpoint_contexts: [EndpointContext32; 31],
    padding: [u8; 0x3E0],
}

#[repr(C)]
pub struct InputContext64 {
    input_control_context: InputControlContext64,
    slot_context: SlotContext64,
    endpoint_contexts: [EndpointContext64; 31],
    padding: [u8; 0x7C0],
}
impl InputContext64 {
    pub fn set_enable_context(&mut self, index: usize, val: bool) {
        self.input_control_context.set_enable_context(index, val);
    }

    pub fn set_route_string(&mut self, route_string: u32) {
        self.slot_context.set_route_string(route_string);
    }
    pub fn set_speed(&mut self, speed: u8) {
        self.slot_context.set_speed(speed);
    }
    pub fn set_context_entries(&mut self, count: u8) {
        self.slot_context.set_context_entries(count);
    }
    pub fn set_route_hub_port_number(&mut self, port_id: u8) {
        self.slot_context.set_route_hub_port_number(port_id);
    }

    pub fn endpoint_context_mut(
        &mut self,
        index: usize,
        is_direction_in: bool,
    ) -> &mut EndpointContext64 {
        &mut self.endpoint_contexts[if index == 0 {
            1
        } else {
            if is_direction_in {
                index * 2
            } else {
                index * 2 + 1
            }
        }]
    }
}

#[repr(C)]
pub struct DeviceContext32 {
    slot_context: SlotContext32,
    endpoint_contexts: [EndpointContext32; 31],
}

#[repr(C)]
pub struct DeviceContext64 {
    slot_context: SlotContext64,
    endpoint_contexts: [EndpointContext64; 31],
}

#[repr(C)]
pub struct InputControlContext32 {
    data: [u32; 8],
}

#[repr(C)]
pub struct SlotContext32 {
    data: [u32; 8],
}

#[repr(C)]
pub struct EndpointContext32 {
    data: [u32; 8],
}

#[repr(C)]
pub struct InputControlContext64 {
    data: [u64; 8],
}
impl InputControlContext64 {
    pub fn set_enable_context(&mut self, index: usize, val: bool) {
        self.data[1] = if val {
            self.data[1] | (1 << index)
        } else {
            self.data[1] & (!(1 << index))
        }
    }
}

#[repr(C)]
pub struct SlotContext64 {
    data: [u64; 8],
}
impl SlotContext64 {
    pub fn set_route_string(&mut self, route_string: u32) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_FFF0_0000) + (route_string as u64 & 0xF_FFFF);
    }
    pub fn set_speed(&mut self, speed: u8) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_FF0F_FFFF) + ((speed as u64) << 20);
    }
    pub fn set_context_entries(&mut self, count: u8) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_07FF_FFFF) + ((count as u64 & 0x1F) << 27);
    }
    pub fn set_route_hub_port_number(&mut self, port_id: u8) {
        self.data[0] = (self.data[1] & 0xFFFF_FFFF_FF00_FFFF) + ((port_id as u64) << 16);
    }
}

#[repr(C)]
pub struct EndpointContext64 {
    data: [u64; 8],
}
impl EndpointContext64 {
    pub fn set_mult(&mut self, mult: u8) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_FF00_FFFE) + ((mult as u64 & 0x3) << 8);
    }
    pub fn set_max_primary_streams(&mut self, count: u8) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_FF00_FFFE) + ((count as u64 & 0x1F) << 10);
    }
    pub fn set_interval(&mut self, interval: u8) {
        self.data[0] = (self.data[0] & 0xFFFF_FFFF_FF00_FFFE) + ((interval as u64) << 16);
    }
    pub fn set_error_count(&mut self, count: u8) {
        self.data[1] = (self.data[1] & 0xFFFF_FFFF_FF00_FFFE) + ((count as u64 & 0x3) << 1);
    }
    pub fn set_endpoint_type(&mut self, r#type: u8) {
        self.data[1] = (self.data[1] & 0xFFFF_FFFF_FFFF_FFC7) + ((r#type as u64 & 0x7) << 3);
    }
    pub fn set_max_burst_size(&mut self, size: u8) {
        self.data[1] = (self.data[1] & 0xFFFF_FFFF_FFFF_00FF) + ((size as u64) << 8);
    }
    pub fn set_max_packet_size(&mut self, size: u16) {
        self.data[1] = (self.data[1] & 0xFFFF_FFFF_0000_FFFF) + ((size as u64) << 16);
    }
    pub fn initialize_dequeue_cycle_state(&mut self) {
        self.data[2] = (self.data[2] & 0xFFFF_FFFF_FFFF_FFFE) + 1;
    }
    pub fn set_dequeue_pointer(&mut self, address: u64) {
        self.data[2] = (self.data[2] & 0xFFFF_FFFF_0000_000F) + (address & 0xFFFF_FFF0);
        self.data[3] = address >> 32;
    }
}
