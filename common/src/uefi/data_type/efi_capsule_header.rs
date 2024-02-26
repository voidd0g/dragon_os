use super::basic_type::{EfiGuid, UnsignedInt32};

#[repr(C)]
pub struct EfiCapsuleHeader {
    capsule_guid: EfiGuid,
    header_size: UnsignedInt32,
    flags: UnsignedInt32,
    capsule_image_size: UnsignedInt32,
}
