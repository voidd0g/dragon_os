use crate::uefi::data_types::basic_types::EFI_GRAPHICS_PIXEL_FORMAT;

pub const PixelRedGreenBlueReserved8BitPerColor: EFI_GRAPHICS_PIXEL_FORMAT = 0;
pub const PixelBlueGreenRedReserved8BitPerColor: EFI_GRAPHICS_PIXEL_FORMAT = 1;
pub const PixelBitMask: EFI_GRAPHICS_PIXEL_FORMAT = 2;
pub const PixelBltOnly: EFI_GRAPHICS_PIXEL_FORMAT = 3;
pub const PixelFormatMax: EFI_GRAPHICS_PIXEL_FORMAT = 4;
