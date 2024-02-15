use crate::uefi::data_type::basic_type::EfiGraphicsOutputBltOperation;

pub const EFI_BLT_VIDEO_FILL: EfiGraphicsOutputBltOperation = 0;
pub const EFI_BLT_VIDEO_TO_BLT_BUFFER: EfiGraphicsOutputBltOperation = 1;
pub const EFI_BLT_BUFFER_TO_VIDEO: EfiGraphicsOutputBltOperation = 2;
pub const EFI_BLT_VIDEO_TO_VIDEO: EfiGraphicsOutputBltOperation = 3;
pub const EFI_GRAPHICS_OUTPUT_BLT_OPERATION_MAX: EfiGraphicsOutputBltOperation = 4;
