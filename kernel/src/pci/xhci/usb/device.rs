use crate::pci::xhci::{software_ring::SoftwareRingManager, TRANSFER_RING_SIZE};

pub struct Device {
    transfer_ring: SoftwareRingManager<TRANSFER_RING_SIZE>,
}
impl Device {
    pub const fn new(transfer_ring: SoftwareRingManager<TRANSFER_RING_SIZE>) -> Self {
        Self { transfer_ring }
    }
}
