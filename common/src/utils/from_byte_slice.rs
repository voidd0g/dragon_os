use crate::uefi::data_types::basic_types::{UINT16, UINT32, UINT64, UINT8, UINTN};

#[deny(non_snake_case)]
pub trait FromByteSlice {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized;
}

#[deny(non_snake_case)]
impl FromByteSlice for UINT8 {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		(*bs.iter().next().unwrap(), &bs[1..])
	}
}

#[deny(non_snake_case)]
impl FromByteSlice for UINT16 {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let (lower, bs) = UINT8::from_byte_slice(bs);
		let (upper, bs) = UINT8::from_byte_slice(bs);
		((lower as UINT16) + ((upper as UINT16) << 8), bs)
	}
}

#[deny(non_snake_case)]
impl FromByteSlice for UINT32 {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let (lower, bs) = UINT16::from_byte_slice(bs);
		let (upper, bs) = UINT16::from_byte_slice(bs);
		((lower as UINT32) + ((upper as UINT32) << 16), bs)
	}
}

#[deny(non_snake_case)]
impl FromByteSlice for UINT64 {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let (lower, bs) = UINT32::from_byte_slice(bs);
		let (upper, bs) = UINT32::from_byte_slice(bs);
		((lower as UINT64) + ((upper as UINT64) << 32), bs)
	}
}

#[deny(non_snake_case)]
impl FromByteSlice for UINTN {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let bytes = UINTN::BITS / 8;
		let mut ret = 0;
		let mut bs = bs;
		for _ in 0..bytes {
			let (byte, new_bs) = UINT8::from_byte_slice(bs);
			bs = new_bs;
			ret <<= 8;
			ret += byte as UINTN;
		}
		(ret, bs)
	}
}

#[deny(non_snake_case)]
impl<T> FromByteSlice for *const T {
	fn from_byte_slice(bs: &[UINT8]) -> (Self, &[UINT8]) where Self: Sized {
		let (address, bs) = UINTN::from_byte_slice(bs);
		(address as *const T, bs)
	}
}