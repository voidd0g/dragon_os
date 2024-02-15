use crate::elf::basic_type::Elf64Flag;

pub const ELF64_FLAG_SPARC_EXT_MASK: Elf64Flag = 0xFFFF00;
pub const ELF64_FLAG_SPARC_32PLUS: Elf64Flag = 0x000100;
pub const ELF64_FLAG_SPARC_SUN_US1: Elf64Flag = 0x000200;
pub const ELF64_FLAG_SPARC_HAL_R1: Elf64Flag = 0x000400;
pub const ELF64_FLAG_SPARC_SUN_US3: Elf64Flag = 0x000800;
pub const ELF64_FLAG_SPARCV9_MM: Elf64Flag = 0x3;
pub const ELF64_FLAG_SPARCV9_TSO: Elf64Flag = 0x0;
pub const ELF64_FLAG_SPARCV9_PSO: Elf64Flag = 0x1;
pub const ELF64_FLAG_SPARCV9_RMO: Elf64Flag = 0x2;
