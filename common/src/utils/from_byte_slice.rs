use crate::uefi::data_types::common_types::UINT8;

pub trait FromByteSlice {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized;
}

impl FromByteSlice for UINT8 {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) {
		(*bs.iter().next().unwrap(), &bs[1..])
	}
}