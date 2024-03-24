use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_LINK};

pub struct LinkTrb {
    next_address: u64,
    toggle_cycle_bit: bool,
}
impl LinkTrb {
    pub fn new(next_address: u64, toggle_cycle_bit: bool) -> Self {
        Self {
            next_address,
            toggle_cycle_bit,
        }
    }
}
impl IntoTransferRequestBlock for LinkTrb {
    fn into_transfer_request_block(self) -> TransferRequestBlock {
        TransferRequestBlock {
            data: [
                (self.next_address & 0xFFFF_FFF0) as u32,
                (self.next_address >> 32) as u32,
                0,
                ((TRB_TYPE_ID_LINK as u32) << 10) + if self.toggle_cycle_bit { 0 } else { 2 },
            ],
        }
    }
}
