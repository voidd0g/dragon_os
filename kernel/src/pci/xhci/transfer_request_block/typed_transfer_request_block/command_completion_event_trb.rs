use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::FromTransferRequestBlock;

pub struct CommandCompletionEventTrb {
    command_trb_pointer: u64,
    command_completion_code: u8,
    slot_id: u8,
}
impl CommandCompletionEventTrb {
    pub fn slot_id(&self) -> u8 {
        self.slot_id
    }
    pub fn command_completion_code(&self) -> u8 {
        self.command_completion_code
    }
    pub fn command_trb_pointer(&self) -> u64 {
        self.command_trb_pointer
    }
}
impl FromTransferRequestBlock for CommandCompletionEventTrb {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self {
        Self {
            command_trb_pointer: (trb.data[0] as u64 & 0xFFFF_FFF0) + ((trb.data[1] as u64) << 32),
            command_completion_code: (trb.data[2] >> 24) as u8,
            slot_id: (trb.data[3] >> 24) as u8,
        }
    }
}

pub const COMMAND_COMPLETION_CODE_SUCCESS: u8 = 1;
