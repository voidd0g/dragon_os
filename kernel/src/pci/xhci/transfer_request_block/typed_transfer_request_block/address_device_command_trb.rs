use crate::pci::xhci::transfer_request_block::TransferRequestBlock;

use super::{IntoTransferRequestBlock, TRB_TYPE_ID_ADDRESS_DEVICE_COMMAND};

pub struct AddressDeviceCommandTrb {
    input_context_address: u64,
    slot_id: u8,
}
impl AddressDeviceCommandTrb {
    pub const fn new(input_context_address: u64, slot_id: u8) -> Self {
        Self {
            input_context_address,
            slot_id,
        }
    }
}
impl IntoTransferRequestBlock for AddressDeviceCommandTrb {
    fn into_transfer_request_block(self) -> TransferRequestBlock {
        TransferRequestBlock {
            data: [
                (self.input_context_address as u32) & 0xFFF_FFF0,
                (self.input_context_address >> 32) as u32,
                0,
                ((TRB_TYPE_ID_ADDRESS_DEVICE_COMMAND as u32) << 10) + ((self.slot_id as u32) << 24),
            ],
        }
    }
}
