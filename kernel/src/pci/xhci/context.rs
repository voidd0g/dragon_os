use core::ptr::{addr_of, addr_of_mut, slice_from_raw_parts, slice_from_raw_parts_mut};

#[repr(align(64))]
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

#[repr(align(64))]
pub struct DeviceContexts<const CONTEXT_COUNT: usize>
where
    [(); (CONTEXT_COUNT + 1) * 8]:,
{
    contexts: [u8; (CONTEXT_COUNT + 1) * 8],
}

impl<'a, const CONTEXT_COUNT: usize> DeviceContexts<CONTEXT_COUNT>
where
    [(); (CONTEXT_COUNT + 1) * 8]:,
{
    pub const fn new() -> Self {
        Self {
            contexts: [0; (CONTEXT_COUNT + 1) * 8],
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

#[repr(C)]
pub struct DeviceContext32 {
    slot_context: SlotContext32,
    endpoint_contexts: [EndpointContext32; 31],
}
impl DeviceContext32 {}

#[repr(C)]
pub struct DeviceContext64 {
    slot_context: SlotContext64,
    endpoint_contexts: [EndpointContext64; 31],
}
impl DeviceContext64 {}

#[repr(C)]
pub struct SlotContext32 {
    data: [u32; 8],
}

#[repr(C)]
pub struct EndpointContext32 {
    data: [u32; 8],
}

#[repr(C)]
pub struct SlotContext64 {
    data: [u32; 16],
}

#[repr(C)]
pub struct EndpointContext64 {
    data: [u32; 16],
}
