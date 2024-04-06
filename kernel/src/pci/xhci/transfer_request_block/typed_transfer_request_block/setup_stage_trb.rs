use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_SETUP_STAGE};

pub struct SetupStageTrb {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
    transfer_type: u8,
}
impl SetupStageTrb {
    pub const fn new(
        request_type: u8,
        request: u8,
        value: u16,
        index: u16,
        length: u16,
        transfer_type: u8,
    ) -> Self {
        Self {
            request_type,
            request,
            value,
            index,
            length,
            transfer_type,
        }
    }
}
impl IntoTransferRequestBlock for SetupStageTrb {
    fn into_transfer_request_block(self) -> TransferRequestBlock {
        TransferRequestBlock {
            data: [
                ((self.value as u32) << 16)
                    + ((self.request as u32) << 8)
                    + (self.request_type as u32),
                ((self.length as u32) << 16) + (self.index as u32),
                0x8,
                ((self.transfer_type as u32 & 0x3) << 16)
                    + ((TRB_TYPE_ID_SETUP_STAGE as u32) << 10)
                    + 0x40,
            ],
        }
    }
}
