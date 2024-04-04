use super::{
    transfer_request_block::{
        typed_transfer_request_block::{link_trb::LinkTrb, OutgoingTypedTransferRequestBlock},
        TransferRequestBlock, TrbArray,
    },
    XhcOperationalRegisters,
};

pub struct SoftwareRingManager<const RING_SIZE: usize>
where
    [(); RING_SIZE]:,
{
    cycle_bit: bool,
    writing_index: usize,
    trbs: SoftwareRingTrbArray<RING_SIZE>,
}

#[repr(align(0x1000))]
pub struct SoftwareRingTrbArray<const RING_SIZE: usize>(TrbArray<RING_SIZE>);
impl<const RING_SIZE: usize> SoftwareRingTrbArray<RING_SIZE> {
    pub const fn new() -> Self {
        Self(TrbArray::new())
    }

    pub fn address(&self) -> u64 {
        self.0.address()
    }

    pub fn put_trb(&mut self, index: usize, cycle_bit: bool, val: TransferRequestBlock) {
        self.0.put_trb(index, cycle_bit, val)
    }
}

impl<const RING_SIZE: usize> SoftwareRingManager<RING_SIZE>
where
    [(); RING_SIZE]:,
{
    pub const fn new() -> Self {
        Self {
            cycle_bit: true,
            writing_index: 0,
            trbs: SoftwareRingTrbArray::new(),
        }
    }

    pub fn initial_dequeue_pointer(&self) -> u64 {
        self.trbs.address()
    }

    pub fn push(&mut self, val: TransferRequestBlock) {
        self.trbs.put_trb(self.writing_index, self.cycle_bit, val);
        self.writing_index += 1;
        if self.writing_index == RING_SIZE - 1 {
            self.trbs.put_trb(
                self.writing_index,
                self.cycle_bit,
                OutgoingTypedTransferRequestBlock::LinkTrb(LinkTrb::new(self.trbs.address(), true))
                    .into_transfer_request_block(),
            );
            self.cycle_bit = !self.cycle_bit;
            self.writing_index = 0;
        }
    }
}
