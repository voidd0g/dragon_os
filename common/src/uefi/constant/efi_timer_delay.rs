#![allow(non_upper_case_globals)]

use crate::uefi::data_types::basic_types::EFI_TIMER_DELAY;

pub const TimerCancel: EFI_TIMER_DELAY = 0;
pub const TimerPeriodic: EFI_TIMER_DELAY = 1;
pub const TimerRelative: EFI_TIMER_DELAY = 2;