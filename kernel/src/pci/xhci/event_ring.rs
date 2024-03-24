use core::{mem::size_of, ptr::addr_of};

use super::{
    transfer_request_block::{TransferRequestBlock, TrbArray},
    XhcInterrupterRegisterSet,
};

#[repr(C)]
struct SegmentTableEntry {
    data: [u32; 4],
}
impl SegmentTableEntry {
    pub const fn new(address: u64, size: u16) -> Self {
        Self {
            data: [
                (address as u32) & 0xFFFF_FFC0,
                (address >> 32) as u32,
                size as u32,
                0,
            ],
        }
    }
}

#[repr(align(0x1000))]
struct SegmentTableEntries<const ENTRY_COUNT: usize> {
    segment_table_entries: [SegmentTableEntry; ENTRY_COUNT],
}

impl<const ENTRY_COUNT: usize> SegmentTableEntries<ENTRY_COUNT> {
    pub const fn new(segment_table_entries: [SegmentTableEntry; ENTRY_COUNT]) -> Self {
        Self {
            segment_table_entries,
        }
    }

    pub fn address(&self) -> u64 {
        addr_of!(self.segment_table_entries) as u64
    }
}

#[repr(align(0x1000))]
pub struct TrbArrays<const ARRAY_SIZE: usize, const ARRAY_COUNT: usize> {
    trbs: [TrbArray<ARRAY_SIZE>; ARRAY_COUNT],
}

impl<const ARRAY_SIZE: usize, const ARRAY_COUNT: usize> TrbArrays<ARRAY_SIZE, ARRAY_COUNT> {
    const RESET_VALUE: TrbArray<ARRAY_SIZE> = TrbArray::new();
    pub const fn new() -> Self {
        Self {
            trbs: [Self::RESET_VALUE; ARRAY_COUNT],
        }
    }

    pub fn address(&self, index: usize) -> u64 {
        addr_of!(self.trbs[index]) as u64
    }
}

pub struct EventRingManagerWithFixedSize<const SEGMENT_SIZE: u16, const SEGMENT_COUNT: u16>
where
    [(); SEGMENT_COUNT as usize]:,
    [(); SEGMENT_SIZE as usize]:,
{
    segment_table: SegmentTableEntries<{ SEGMENT_COUNT as usize }>,
    trb_arrays: TrbArrays<{ SEGMENT_SIZE as usize }, { SEGMENT_COUNT as usize }>,
    interrupter_register_set: XhcInterrupterRegisterSet,
    cycle_bit: bool,
    segment_index: usize,
}

impl<const SEGMENT_SIZE: u16, const SEGMENT_COUNT: u16>
    EventRingManagerWithFixedSize<SEGMENT_SIZE, SEGMENT_COUNT>
where
    [(); SEGMENT_COUNT as usize]:,
    [(); SEGMENT_SIZE as usize]:,
{
    pub fn new(interrupter_register_set: XhcInterrupterRegisterSet) -> Self {
        let trb_arrays = TrbArrays::new();
        const SEGMENT_TABLE_RESET_VALUE: SegmentTableEntry = SegmentTableEntry::new(0, 0);
        let mut segment_table = [SEGMENT_TABLE_RESET_VALUE; SEGMENT_COUNT as usize];
        for i in 0..SEGMENT_COUNT {
            segment_table[i as usize] =
                SegmentTableEntry::new(trb_arrays.address(i as usize), SEGMENT_SIZE);
        }
        Self {
            trb_arrays,
            segment_table: SegmentTableEntries::new(segment_table),
            interrupter_register_set,
            cycle_bit: true,
            segment_index: 0,
        }
    }

    pub fn initialize(&self) {
        self.interrupter_register_set
            .set_event_ring_segment_table_size(SEGMENT_COUNT);
        self.interrupter_register_set
            .set_event_ring_dequeue_pointer(self.trb_arrays.address(0));
        self.interrupter_register_set
            .set_event_ring_segment_table_base_address(self.segment_table.address());
        self.interrupter_register_set
            .set_interrupt_pending_and_enable();
    }

    pub fn front(&self) -> Option<TransferRequestBlock> {
        let dequeue_poiner =
            self.interrupter_register_set.event_ring_dequeue_pointer() & 0xFFFF_FFFF_FFFF_FFF0;
        let front = unsafe { (dequeue_poiner as *const TransferRequestBlock).read() };
        if front.cycle_bit() == self.cycle_bit {
            Some(front)
        } else {
            None
        }
    }

    pub fn pop(&mut self) -> Option<TransferRequestBlock> {
        let dequeue_poiner =
            self.interrupter_register_set.event_ring_dequeue_pointer() & 0xFFFF_FFFF_FFFF_FFF0;
        let front = unsafe { (dequeue_poiner as *const TransferRequestBlock).read() };
        if front.cycle_bit() == self.cycle_bit {
            let mut next_dequeue_poiner = dequeue_poiner + size_of::<TransferRequestBlock>() as u64;
            if next_dequeue_poiner
                == self.trb_arrays.address(self.segment_index)
                    + (size_of::<TransferRequestBlock>() * SEGMENT_SIZE as usize) as u64
            {
                self.segment_index += 1;
                if self.segment_index == SEGMENT_COUNT as usize {
                    self.segment_index = 0;
                    self.cycle_bit = !self.cycle_bit;
                }
                next_dequeue_poiner = self.trb_arrays.address(self.segment_index);
            }
            self.interrupter_register_set
                .set_event_ring_dequeue_pointer(next_dequeue_poiner);

            Some(front)
        } else {
            None
        }
    }
}
