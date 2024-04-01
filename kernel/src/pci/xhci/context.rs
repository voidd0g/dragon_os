use core::{
    mem::size_of,
    ptr::{addr_of, addr_of_mut, slice_from_raw_parts, slice_from_raw_parts_mut},
};

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
