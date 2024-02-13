use crate::uefi::data_types::basic_types::EFI_GRAPHICS_OUTPUT_BLT_OPERATION;

pub const EfiBltVideoFill: EFI_GRAPHICS_OUTPUT_BLT_OPERATION = 0;
pub const EfiBltVideoToBltBuffer: EFI_GRAPHICS_OUTPUT_BLT_OPERATION = 1;
pub const EfiBltBufferToVideo: EFI_GRAPHICS_OUTPUT_BLT_OPERATION = 2;
pub const EfiBltVideoToVideo: EFI_GRAPHICS_OUTPUT_BLT_OPERATION = 3;
pub const EfiGraphicsOutputBltOperationMax: EFI_GRAPHICS_OUTPUT_BLT_OPERATION = 4;