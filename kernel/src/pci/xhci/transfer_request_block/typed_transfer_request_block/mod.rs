pub mod address_device_command_trb;
pub mod command_completion_event_trb;
pub mod disable_slot_command_trb;
pub mod enable_slot_command_trb;
pub mod link_trb;
pub mod port_status_chage_event_trb;
pub mod setup_stage_trb;
pub mod transfer_event_trb;

use self::{
    address_device_command_trb::AddressDeviceCommandTrb,
    command_completion_event_trb::CommandCompletionEventTrb,
    disable_slot_command_trb::DisableSlotCommandTrb, enable_slot_command_trb::EnableSlotCommandTrb,
    link_trb::LinkTrb, port_status_chage_event_trb::PortStatusChangeEventTrb,
    setup_stage_trb::SetupStageTrb, transfer_event_trb::TransferEventTrb,
};

use super::TransferRequestBlock;

pub enum EventRingTypedTransferRequestBlock {
    TransferEventTrb(TransferEventTrb),
    CommandCompletionEventTrb(CommandCompletionEventTrb),
    PortStatusChangeEventTrb(PortStatusChangeEventTrb),
}

impl EventRingTypedTransferRequestBlock {
    pub fn from_transfer_request_block(trb: TransferRequestBlock) -> Result<Self, ()> {
        match trb.trb_type() {
            TRB_TYPE_ID_TRANSFER_EVENT => Ok(Self::TransferEventTrb(
                TransferEventTrb::from_transfer_request_block(trb),
            )),
            TRB_TYPE_ID_COMMAND_COMPLETION_EVENT => Ok(Self::CommandCompletionEventTrb(
                CommandCompletionEventTrb::from_transfer_request_block(trb),
            )),
            TRB_TYPE_ID_PORT_STATUS_CHANGE_EVENT => Ok(Self::PortStatusChangeEventTrb(
                PortStatusChangeEventTrb::from_transfer_request_block(trb),
            )),
            _ => Err(()),
        }
    }
}

pub enum CommandRingTypedTransferRequestBlock {
    EnableSlotCommandTrb(EnableSlotCommandTrb),
    DisableSlotCommandTrb(DisableSlotCommandTrb),
    AddressDeviceCommandTrb(AddressDeviceCommandTrb),
}

impl CommandRingTypedTransferRequestBlock {
    pub fn into_transfer_request_block(self) -> TransferRequestBlock {
        match self {
            Self::EnableSlotCommandTrb(enable_slot_command_trb) => {
                enable_slot_command_trb.into_transfer_request_block()
            }
            Self::DisableSlotCommandTrb(disable_slot_command_trb) => {
                disable_slot_command_trb.into_transfer_request_block()
            }
            Self::AddressDeviceCommandTrb(address_device_command_trb) => {
                address_device_command_trb.into_transfer_request_block()
            }
        }
    }
}

pub enum TransferRingTypedTransferRequestBlock {
    SetupStageTrb(SetupStageTrb),
}

impl TransferRingTypedTransferRequestBlock {
    pub fn into_transfer_request_block(self) -> TransferRequestBlock {
        match self {
            Self::SetupStageTrb(setup_stage_trb) => setup_stage_trb.into_transfer_request_block(),
        }
    }
}

pub const TRB_TYPE_ID_SETUP_STAGE: u8 = 2;
pub const TRB_TYPE_ID_DATA_STAGE: u8 = 3;
pub const TRB_TYPE_ID_STATUS_STAGE: u8 = 4;
pub const TRB_TYPE_ID_LINK: u8 = 6;
pub const TRB_TYPE_ID_ENABLE_SLOT_COMMAND: u8 = 9;
pub const TRB_TYPE_ID_DISABLE_SLOT_COMMAND: u8 = 10;
pub const TRB_TYPE_ID_ADDRESS_DEVICE_COMMAND: u8 = 11;
const TRB_TYPE_ID_TRANSFER_EVENT: u8 = 32;
const TRB_TYPE_ID_COMMAND_COMPLETION_EVENT: u8 = 33;
const TRB_TYPE_ID_PORT_STATUS_CHANGE_EVENT: u8 = 34;

pub trait IntoTransferRequestBlock {
    fn into_transfer_request_block(self) -> TransferRequestBlock;
}
pub trait FromTransferRequestBlock {
    fn from_transfer_request_block(trb: TransferRequestBlock) -> Self;
}
