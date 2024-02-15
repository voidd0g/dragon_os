use crate::uefi::data_type::basic_type::EfiTimerDelay;

pub const TIMER_CANCEL: EfiTimerDelay = 0;
pub const TIMER_PERIODIC: EfiTimerDelay = 1;
pub const TIMER_RELATIVE: EfiTimerDelay = 2;
