pub mod link_trb;
pub mod transfer_event_trb;

use self::{link_trb::LinkTrb, transfer_event_trb::TransferEventTrb};

use super::TransferRequestBlock;

pub enum TypedTransferRequestBlock {
    LinkTrb(LinkTrb),
    TransferEventTrb(TransferEventTrb),
}

impl TypedTransferRequestBlock {
    pub fn from_transfer_request_block(trb: TransferRequestBlock) -> Result<Self, ()> {
        match trb.trb_type() {
            TRB_TYPE_ID_TRANSFER_EVENT => Ok(TypedTransferRequestBlock::TransferEventTrb(TransferEventTrb::from_transfer_request_block(trb))),
            _ => Err(()),
        }
    }

    pub fn into_transfer_request_block(self) -> Result<TransferRequestBlock, ()> {
        match self {
            TypedTransferRequestBlock::LinkTrb(link_trb) => Ok(link_trb.into_transfer_request_block()),
            _ => Err(())
        }
    }
}

const TRB_TYPE_ID_LINK: u8 = 6;
const TRB_TYPE_ID_TRANSFER_EVENT: u8 = 32;
const TRB_TYPE_ID_COMMAND_COMPLETION_EVENT: u8 = 33;
const TRB_TYPE_ID_PORT_STATUS_CHANGE_EVENT: u8 = 34;

pub trait IntoTransferRequestBlock {
    fn into_transfer_request_block(self) -> TransferRequestBlock;
}
pub trait FromTransferRequestBlock {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self;
}
