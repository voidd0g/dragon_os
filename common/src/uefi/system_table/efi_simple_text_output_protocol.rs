use crate::uefi::data_types::{CHAR16, EFI_STATUS};

pub struct EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    Reset: extern "efiapi" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, ExtendedVerification: bool) -> EFI_STATUS,
    OutputString: extern "efiapi" fn(This: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL, String: *const CHAR16) -> EFI_STATUS,
    _Unuse0: u64,
    _Unuse1: u64,
    _Unuse2: u64,
    _Unuse3: u64,
    _Unuse4: u64,
    _Unuse5: u64,
    _Unuse6: u64,
    _Unuse7: u64,
}

impl EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL {
    pub fn Reset(&self, ExtendedVerification: bool) -> EFI_STATUS {
        unsafe {(self.Reset)(self, ExtendedVerification)}
    }

    pub fn OutputString(&self, String: *const CHAR16) -> EFI_STATUS {
        unsafe{(self.OutputString)(self, String)}
    }
}