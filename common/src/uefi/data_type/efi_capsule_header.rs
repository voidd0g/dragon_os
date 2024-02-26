use super::basic_type::EfiGuid;

#[repr(C)]
pub struct EfiCapsuleHeader {
    capsule_guid: EfiGuid,
    header_size: u32,
    flags: u32,
    capsule_image_size: u32,
}
