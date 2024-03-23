use super::{transfer_request_block::TrbArray, XhcOperationalRegisters};

pub struct SoftwareRingManager<const COMMAND_RING_SIZE: usize>
where
    [(); COMMAND_RING_SIZE]:,
{
    cycle_bit: bool,
    writing_index: usize,
    trbs: SoftwareRingTrbArray<COMMAND_RING_SIZE>,
    operational_registers: XhcOperationalRegisters,
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
}

impl<const RING_SIZE: usize> SoftwareRingManager<RING_SIZE>
where
    [(); RING_SIZE]:,
{
    pub const fn new(operational_registers: XhcOperationalRegisters) -> Self {
        Self {
            cycle_bit: true,
            writing_index: 0,
            trbs: SoftwareRingTrbArray::new(),
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
