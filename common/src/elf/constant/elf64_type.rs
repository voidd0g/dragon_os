use crate::elf::basic_type::Elf64Type;

pub const ELF64_TYPE_NONE: Elf64Type = 0;
pub const ELF64_TYPE_RELOCATABLE: Elf64Type = 1;
pub const ELF64_TYPE_EXECUTABLE: Elf64Type = 2;
pub const ELF64_TYPE_DYNAMIC: Elf64Type = 3;
pub const ELF64_TYPE_CORE: Elf64Type = 4;
pub const ELF64_TYPE_LOW_PROCESSOR: Elf64Type = 0xff00;
pub const ELF64_TYPE_HIGH_PROCESSOR: Elf64Type = 0xffff;
