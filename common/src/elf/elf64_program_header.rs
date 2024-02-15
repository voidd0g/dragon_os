use super::basic_type::{Elf64Address, Elf64Offset, Elf64ProgramType, Elf64XWord};

#[repr(C)]
pub struct Elf64ProgramHeader {
    r#type: Elf64ProgramType,
    offset: Elf64Offset,
    virtual_address: Elf64Address,
    phisical_address: Elf64Address,
    file_size: Elf64XWord,
    memory_size: Elf64XWord,
    align: Elf64XWord,
}

impl Elf64ProgramHeader {
    pub fn r#type(&self) -> Elf64ProgramType {
        self.r#type
    }
    pub fn offset(&self) -> Elf64Offset {
        self.offset
    }
    pub fn virtual_address(&self) -> Elf64Address {
        self.virtual_address
    }

    pub fn file_size(&self) -> Elf64XWord {
        self.file_size
    }
    pub fn memory_size(&self) -> Elf64XWord {
        self.memory_size
    }
}
