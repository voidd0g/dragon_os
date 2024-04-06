use crate::{
    pci::xhci::{
        software_ring::SoftwareRingManager,
        transfer_request_block::typed_transfer_request_block::{
            setup_stage_trb::SetupStageTrb, TransferRingTypedTransferRequestBlock,
        },
        XhcDoorbellRegisters,
    },
    services::Services,
};

const TRANSFER_RING_SIZE: usize = 32;
const DESCRIPTOR_BUFFER_SIZE: usize = 256;
pub struct Device {
    port_id: u8,
    slot_id: u8,
    doorbell_registers: XhcDoorbellRegisters,
    transfer_ring: SoftwareRingManager<TRANSFER_RING_SIZE>,
    descriptor_buffer: [u8; DESCRIPTOR_BUFFER_SIZE],
}
impl Device {
    pub const fn new(port_id: u8, slot_id: u8, doorbell_registers: XhcDoorbellRegisters) -> Self {
        Self {
            port_id,
            slot_id,
            doorbell_registers,
            transfer_ring: SoftwareRingManager::new(),
            descriptor_buffer: [0; DESCRIPTOR_BUFFER_SIZE],
        }
    }

    pub fn port_id(&self) -> u8 {
        self.port_id
    }

    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }

    pub fn initial_dequeue_pointer(&self) -> u64 {
        self.transfer_ring.initial_dequeue_pointer()
    }

    pub fn start_initialize(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        Ok(())
    }

    fn get_descriptor(&mut self, services: &Services, height: &mut u32) -> Result<(), ()> {
        self.transfer_ring
            .push(TransferRingTypedTransferRequestBlock::SetupStageTrb(
                SetupStageTrb::new(request_type, request, value, index, length, transfer_type),
            ))
    }
}
