use core::ptr::addr_of;

#[repr(C)]
pub struct TransferRequestBlock {
    data: [u32; 4],
}

impl TransferRequestBlock {
    pub const fn new() -> Self {
        Self { data: [0; 4] }
    }

    pub const fn link_trb(address: u64, cycle_bit: bool, toggle_cb: bool) -> Self {
        Self {
            data: [
                (address & 0xFFFF_FFF0) as u32,
                (address >> 32) as u32,
                0,
                0x0000_1800 + if toggle_cb { 0 } else { 2 } + if cycle_bit { 0 } else { 1 },
            ],
        }
    }
}

#[repr(align(64))]
pub struct TrbArray<const ARRAY_SIZE: usize>
where
    [(); ARRAY_SIZE]:,
{
    trbs: [TransferRequestBlock; ARRAY_SIZE],
}
impl<const ARRAY_SIZE: usize> TrbArray<ARRAY_SIZE>
where
    [(); ARRAY_SIZE]:,
{
    pub const fn new() -> Self {
        const INIT_VAL: TransferRequestBlock = TransferRequestBlock::new();
        Self {
            trbs: [INIT_VAL; ARRAY_SIZE],
        }
    }

    pub fn address(&self) -> u64 {
        addr_of!(self.trbs) as u64
    }
}
