use crate::uefi::data_types::basic_types::UINT64;

pub const EFI_MEMORY_UC: UINT64 = 0x0000000000000001;
pub const EFI_MEMORY_WC: UINT64 = 0x0000000000000002;
pub const EFI_MEMORY_WT: UINT64 = 0x0000000000000004;
pub const EFI_MEMORY_WB: UINT64 = 0x0000000000000008;
pub const EFI_MEMORY_UCE: UINT64 = 0x0000000000000010;
pub const EFI_MEMORY_WP: UINT64 = 0x0000000000001000;
pub const EFI_MEMORY_RP: UINT64 = 0x0000000000002000;
pub const EFI_MEMORY_XP: UINT64 = 0x0000000000004000;
pub const EFI_MEMORY_NV: UINT64 = 0x0000000000008000;
pub const EFI_MEMORY_MORE_RELIABLE: UINT64 = 0x0000000000010000;
pub const EFI_MEMORY_RO: UINT64 = 0x0000000000020000;
pub const EFI_MEMORY_SP: UINT64 = 0x0000000000040000;
pub const EFI_MEMORY_CPU_CRYPTO: UINT64 = 0x0000000000080000;
pub const EFI_MEMORY_RUNTIME: UINT64 = 0x8000000000000000;
pub const EFI_MEMORY_ISA_VALID: UINT64 = 0x4000000000000000;
pub const EFI_MEMORY_ISA_MASK: UINT64 = 0x0FFFF00000000000;