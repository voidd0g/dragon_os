use crate::uefi::data_type::basic_type::EFI_TIMER_DELAY;

pub const TIMER_CANCEL: EFI_TIMER_DELAY = 0;
pub const TIMER_PERIODIC: EFI_TIMER_DELAY = 1;
pub const TIMER_RELATIVE: EFI_TIMER_DELAY = 2;
