pub mod enable_slot_command_trb;
pub mod link_trb;
pub mod port_status_chage_event_trb;
pub mod transfer_event_trb;

use self::{
    enable_slot_command_trb::EnableSlotCommandTrb, link_trb::LinkTrb,
    port_status_chage_event_trb::PortStatusChangeEventTrb, transfer_event_trb::TransferEventTrb,
};

use super::TransferRequestBlock;

pub enum IncomingTypedTransferRequestBlock {
    TransferEventTrb(TransferEventTrb),
    PortStatusChangeEventTrb(PortStatusChangeEventTrb),
}

impl IncomingTypedTransferRequestBlock {
    pub fn from_transfer_request_block(trb: TransferRequestBlock) -> Result<Self, ()> {
        match trb.trb_type() {
            TRB_TYPE_ID_TRANSFER_EVENT => Ok(IncomingTypedTransferRequestBlock::TransferEventTrb(
                TransferEventTrb::from_transfer_request_block(trb),
            )),
            TRB_TYPE_ID_PORT_STATUS_CHANGE_EVENT => {
                Ok(IncomingTypedTransferRequestBlock::PortStatusChangeEventTrb(
                    PortStatusChangeEventTrb::from_transfer_request_block(trb),
                ))
            }
            _ => Err(()),
        }
    }
}

pub enum OutgoingTypedTransferRequestBlock {
    LinkTrb(LinkTrb),
    EnableSlotCommandTrb(EnableSlotCommandTrb),
}

impl OutgoingTypedTransferRequestBlock {
    pub fn into_transfer_request_block(self) -> TransferRequestBlock {
        match self {
            OutgoingTypedTransferRequestBlock::LinkTrb(link_trb) => {
                link_trb.into_transfer_request_block()
            }
            OutgoingTypedTransferRequestBlock::EnableSlotCommandTrb(enable_slot_command_trb) => {
                enable_slot_command_trb.into_transfer_request_block()
            }
        }
    }
}

const TRB_TYPE_ID_LINK: u8 = 6;
const TRB_TYPE_ID_ENABLE_SLOT_COMMAND: u8 = 9;
const TRB_TYPE_ID_TRANSFER_EVENT: u8 = 32;
const TRB_TYPE_ID_COMMAND_COMPLETION_EVENT: u8 = 33;
const TRB_TYPE_ID_PORT_STATUS_CHANGE_EVENT: u8 = 34;

pub trait IntoTransferRequestBlock {
    fn into_transfer_request_block(self) -> TransferRequestBlock;
}
pub trait FromTransferRequestBlock {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self;
}
