use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_SETUP_STAGE};

pub struct SetupStageTrb {
    request_type: u8,
    request: u8,
    value: u16,
    index: u16,
    length: u16,
}
impl SetupStageTrb {
    // pub const fn new(slot_id: u8) -> Self {
    //     Self { slot_id }
    // }
}
// impl IntoTransferRequestBlock for SetupStageTrb {
//     fn into_transfer_request_block(self) -> TransferRequestBlock {
//         TransferRequestBlock {
//             data: [
//                 0,
//                 0,
//                 0,
//                 ((TRB_TYPE_ID_SETUP_STAGE as u32) << 10) + ((self.slot_id as u32) << 24),
//             ],
//         }
//     }
// }