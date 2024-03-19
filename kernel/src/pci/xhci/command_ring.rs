use super::transfer_request_block::TrbArray;

pub struct CommandRingManager<const COMMAND_RING_SIZE: usize>
where
    [(); COMMAND_RING_SIZE]:,
{
    cycle_bit: bool,
    writing_index: usize,
    trbs: TrbArray<COMMAND_RING_SIZE>,
}

impl<const COMMAND_RING_SIZE: usize> CommandRingManager<COMMAND_RING_SIZE>
where
    [(); COMMAND_RING_SIZE]:,
{
    pub const fn new() -> Self {
        Self {
            cycle_bit: true,
            writing_index: 0,
            trbs: TrbArray::new(),
        }
    }

    pub fn get_crcr_value_to_set(&self, prev_val: u64) -> u64 {
        prev_val
            & 0x30 + self.trbs.address()
            & 0xFFFF_FFFF_FFFF_FFC0 + if self.cycle_bit { 1 } else { 0 }
    }
}
