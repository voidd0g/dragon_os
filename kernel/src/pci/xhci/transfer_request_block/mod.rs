pub mod typed_transfer_request_block;

use core::ptr::addr_of;

#[repr(C)]
pub struct TransferRequestBlock {
    data: [u32; 4],
}

impl TransferRequestBlock {
    pub const fn new() -> Self {
        Self { data: [0; 4] }
    }

    pub fn cycle_bit(&self) -> bool {
        self.data[3] & 0x1 != 0
    }

    pub fn trb_type(&self) -> u8 {
        ((self.data[3] >> 10) & 0x3F) as u8
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

    pub fn put_trb(&mut self, index: usize, cycle_bit: bool, mut val: TransferRequestBlock) {
        val.data[3] = (val.data[3] & 0xFFFF_FFFE) + if cycle_bit { 1 } else { 0 };
        let target = &mut self.trbs[index];
        for i in 0..3 {
            target.data[i] = val.data[i];
        }
    }
}
