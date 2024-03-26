use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_ENABLE_SLOT_COMMAND};

pub struct EnableSlotCommandTrb {}
impl EnableSlotCommandTrb {
    pub const fn new() -> Self {
        Self {}
    }
}
impl IntoTransferRequestBlock for EnableSlotCommandTrb {
    fn into_transfer_request_block(self) -> TransferRequestBlock {
        TransferRequestBlock {
            data: [0, 0, 0, (TRB_TYPE_ID_ENABLE_SLOT_COMMAND as u32) << 10],
        }
    }
}
