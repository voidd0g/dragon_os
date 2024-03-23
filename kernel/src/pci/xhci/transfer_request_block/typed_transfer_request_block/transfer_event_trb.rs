use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::FromTransferRequestBlock;

pub struct TransferEventTrb {
    slot_id: u8,
    transfer_length: u32,
    completion_code: u8,
    endpoint_id: u8,
    is_event_data: bool,
    trb_pointer: u64,
}
impl TransferEventTrb {
    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }
    pub fn transfer_length(&self) -> u32 {
        self.transfer_length
    }
    pub fn completion_code(&self) -> u8 {
        self.completion_code
    }
    pub fn endpoint_id(&self) -> u8 {
        self.endpoint_id
    }
}
impl FromTransferRequestBlock for TransferEventTrb {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self {
        Self {
            slot_id: (trb.data[3] >> 24) as u8,
            transfer_length: trb.data[2] & 0x00FF_FFFF,
            completion_code: (trb.data[2] >> 24) as u8,
            endpoint_id: ((trb.data[2] >> 16) & 0x1F) as u8,
            is_event_data: (trb.data[3] & 0x0000_0004) != 0,
            trb_pointer: (trb.data[0] as u64) + ((trb.data[1] as u64) << 32),
        }
    }
}
