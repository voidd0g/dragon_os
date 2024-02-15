use crate::uefi::data_type::basic_type::EfiLocateSearchType;

pub const ALL_HANDLES: EfiLocateSearchType = 0;
pub const BY_REGISTER_NOTIFY: EfiLocateSearchType = 1;
pub const BY_PROTOCOL: EfiLocateSearchType = 2;
