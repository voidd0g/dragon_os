use core::ptr::{addr_of, slice_from_raw_parts};

use super::transfer_request_block::{TransferRequestBlock, TrbArray};

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

#[repr(align(64))]
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

#[repr(align(64))]
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
}

impl<const SEGMENT_SIZE: u16, const SEGMENT_COUNT: u16>
    EventRingManagerWithFixedSize<SEGMENT_SIZE, SEGMENT_COUNT>
where
    [(); SEGMENT_COUNT as usize]:,
    [(); SEGMENT_SIZE as usize]:,
{
    pub fn new() -> Self {
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
        }
    }
}

impl<const SEGMENT_SIZE: u16, const SEGMENT_COUNT: u16> EventRingManager
    for EventRingManagerWithFixedSize<SEGMENT_SIZE, SEGMENT_COUNT>
where
    [(); SEGMENT_COUNT as usize]:,
    [(); SEGMENT_SIZE as usize]:,
{
    fn segment_table_size(&self) -> u16 {
        SEGMENT_COUNT
    }

    fn deque_pointer(&self) -> u64 {
        self.trb_arrays.address(0)
    }

    fn segment_table_base_address(&self) -> u64 {
        self.segment_table.address()
    }
}

pub trait EventRingManager {
    fn segment_table_size(&self) -> u16;
    fn deque_pointer(&self) -> u64;
    fn segment_table_base_address(&self) -> u64;
}
