use super::{transfer_request_block::TrbArray, XhcOperationalRegisters};

pub struct CommandRingManager<const COMMAND_RING_SIZE: usize>
where
    [(); COMMAND_RING_SIZE]:,
{
    cycle_bit: bool,
    writing_index: usize,
    trbs: TrbArray<COMMAND_RING_SIZE>,
    operational_registers: XhcOperationalRegisters,
}

impl<const COMMAND_RING_SIZE: usize> CommandRingManager<COMMAND_RING_SIZE>
where
    [(); COMMAND_RING_SIZE]:,
{
    pub const fn new(operational_registers: XhcOperationalRegisters) -> Self {
        Self {
            cycle_bit: true,
            writing_index: 0,
            trbs: TrbArray::new(),
            operational_registers,
        }
    }

    pub fn initialize(&self) {
        self.operational_registers
            .set_command_ring_control_register(
                self.operational_registers.command_ring_control_register()
                    & 0x30 + self.trbs.address()
                    & 0xFFFF_FFFF_FFFF_FFC0 + if self.cycle_bit { 1 } else { 0 },
            );
    }
}
