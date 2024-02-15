use crate::elf::basic_type::Elf64ProgramType;

pub const ELF64_PROGRAM_TYPE_NULL: Elf64ProgramType = 0;
pub const ELF64_PROGRAM_TYPE_LOAD: Elf64ProgramType = 1;
pub const ELF64_PROGRAM_TYPE_DYNAMIC: Elf64ProgramType = 2;
pub const ELF64_PROGRAM_TYPE_INTERP: Elf64ProgramType = 3;
pub const ELF64_PROGRAM_TYPE_NOTE: Elf64ProgramType = 4;
pub const ELF64_PROGRAM_TYPE_SHLIB: Elf64ProgramType = 5;
pub const ELF64_PROGRAM_TYPE_PHDR: Elf64ProgramType = 6;
pub const ELF64_PROGRAM_TYPE_TLS: Elf64ProgramType = 7;
pub const ELF64_PROGRAM_TYPE_LOSUNW: Elf64ProgramType = 0x6ffffffa;
pub const ELF64_PROGRAM_TYPE_SUNWBSS: Elf64ProgramType = 0x6ffffffa;
pub const ELF64_PROGRAM_TYPE_SUNWSTACK: Elf64ProgramType = 0x6ffffffb;
pub const ELF64_PROGRAM_TYPE_HISUNW: Elf64ProgramType = 0x6fffffff;
pub const ELF64_PROGRAM_TYPE_LOPROC: Elf64ProgramType = 0x70000000;
pub const ELF64_PROGRAM_TYPE_HIPROC: Elf64ProgramType = 0x7fffffff;
