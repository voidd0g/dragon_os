use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_DISABLE_SLOT_COMMAND};

pub struct DisableSlotCommandTrb {
    slot_id: u8,
}
impl DisableSlotCommandTrb {
    pub const fn new(slot_id: u8) -> Self {
        Self { slot_id }
    }
}
impl IntoTransferRequestBlock for DisableSlotCommandTrb {
    fn into_transfer_request_block(self) -> TransferRequestBlock {
        TransferRequestBlock {
            data: [
                0,
                0,
                0,
                ((TRB_TYPE_ID_DISABLE_SLOT_COMMAND as u32) << 10) + ((self.slot_id as u32) << 24),
            ],
        }
    }
}
