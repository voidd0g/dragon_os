use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::FromTransferRequestBlock;

pub struct PortStatusChangeEventTrb {
    port_id: u8,
    completion_code: u8,
}
impl PortStatusChangeEventTrb {
    pub fn port_id(&self) -> u8 {
        self.port_id
    }
    pub fn completion_code(&self) -> u8 {
        self.completion_code
    }
}
impl FromTransferRequestBlock for PortStatusChangeEventTrb {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self {
        Self {
            port_id: (trb.data[0] >> 24) as u8,
            completion_code: (trb.data[2] >> 24) as u8,
        }
    }
}
