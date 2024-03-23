use core::ptr::{addr_of, addr_of_mut, slice_from_raw_parts, slice_from_raw_parts_mut};

#[repr(align(0x1000))]
pub struct DeviceContextBaseAddressArray<const ARRAY_SIZE: usize>
where
    [(); ARRAY_SIZE + 1]:,
{
    dcbaa: [usize; ARRAY_SIZE + 1],
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

    pub unsafe fn pointer(&self) -> *const [usize; ARRAY_SIZE + 1] {
        addr_of!(self.dcbaa)
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

    pub fn device_contexts_32(&'a self) -> &'a [DeviceContext32] {
        unsafe {
            slice_from_raw_parts(
                addr_of!(self.contexts) as *const DeviceContext32,
                CONTEXT_COUNT,
            )
            .as_ref()
            .unwrap()
        }
    }

    pub fn device_contexts_64(&'a self) -> &'a [DeviceContext64] {
        unsafe {
            slice_from_raw_parts(
                addr_of!(self.contexts) as *const DeviceContext64,
                CONTEXT_COUNT,
            )
            .as_ref()
            .unwrap()
        }
    }

    pub fn device_contexts_32_mut(&'a mut self) -> &'a mut [DeviceContext32] {
        unsafe {
            slice_from_raw_parts_mut(
                addr_of_mut!(self.contexts) as *mut DeviceContext32,
                CONTEXT_COUNT,
            )
            .as_mut()
            .unwrap()
        }
    }

    pub fn device_contexts_64_mut(&'a mut self) -> &'a mut [DeviceContext64] {
        unsafe {
            slice_from_raw_parts_mut(
                addr_of_mut!(self.contexts) as *mut DeviceContext64,
                CONTEXT_COUNT,
            )
            .as_mut()
            .unwrap()
        }
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

    pub fn device_contexts_32(&'a self) -> &'a [InputContext32] {
        unsafe {
            slice_from_raw_parts(
                addr_of!(self.contexts) as *const InputContext32,
                CONTEXT_COUNT,
            )
            .as_ref()
            .unwrap()
        }
    }

    pub fn device_contexts_64(&'a self) -> &'a [InputContext64] {
        unsafe {
            slice_from_raw_parts(
                addr_of!(self.contexts) as *const InputContext64,
                CONTEXT_COUNT,
            )
            .as_ref()
            .unwrap()
        }
    }

    pub fn device_contexts_32_mut(&'a mut self) -> &'a mut [InputContext32] {
        unsafe {
            slice_from_raw_parts_mut(
                addr_of_mut!(self.contexts) as *mut InputContext32,
                CONTEXT_COUNT,
            )
            .as_mut()
            .unwrap()
        }
    }

    pub fn device_contexts_64_mut(&'a mut self) -> &'a mut [InputContext64] {
        unsafe {
            slice_from_raw_parts_mut(
                addr_of_mut!(self.contexts) as *mut InputContext64,
                CONTEXT_COUNT,
            )
            .as_mut()
            .unwrap()
        }
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
    data: [u32; 16],
}

#[repr(C)]
pub struct SlotContext64 {
    data: [u32; 16],
}

#[repr(C)]
pub struct EndpointContext64 {
    data: [u32; 16],
}
