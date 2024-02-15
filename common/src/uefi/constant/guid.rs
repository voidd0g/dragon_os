use crate::uefi::data_type::basic_type::EfiGuid;

pub const EFI_LOADED_IMAGE_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x5B1B31A1,
    0x9562,
    0x11d2,
    [0x8E, 0x3F, 0x00, 0xA0, 0xC9, 0x69, 0x72, 0x3B],
);
pub const EFI_DEVICE_PATH_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x09576e91,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_SIMPLE_TEXT_INPUT_EX_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0xdd9e7534,
    0x7762,
    0x4698,
    [0x8c, 0x14, 0xf5, 0x85, 0x17, 0xa6, 0x25, 0xaa],
);
pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x387477c1,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x387477c2,
    0x69c7,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x0964e5b22,
    0x6459,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_FILE_INFO_GUID: EfiGuid = EfiGuid::new(
    0x09576e92,
    0x6d3f,
    0x11d2,
    [0x8e, 0x39, 0x00, 0xa0, 0xc9, 0x69, 0x72, 0x3b],
);
pub const EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid::new(
    0x9042a9de,
    0x23dc,
    0x4a38,
    [0x96, 0xfb, 0x7a, 0xde, 0xd0, 0x80, 0x51, 0x6a],
);
