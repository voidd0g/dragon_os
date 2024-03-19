use super::basic_type::{Elf64Address, Elf64Offset, Elf64SectionType, Elf64Word, Elf64XWord};

#[repr(C)]
pub struct Elf64SectionHeader {
    name: Elf64Word,
    r#type: Elf64SectionType,
    flags: Elf64XWord,
    address: Elf64Address,
    offset: Elf64Offset,
    size: Elf64XWord,
    link: Elf64Word,
    info: Elf64Word,
    address_align: Elf64XWord,
    entry_size: Elf64XWord,
}

impl Elf64SectionHeader {
    pub fn r#type(&self) -> Elf64SectionType {
        self.r#type
    }
    pub fn offset(&self) -> Elf64Offset {
        self.offset
    }
    pub fn size(&self) -> Elf64XWord {
        self.size
    }
    pub fn entry_size(&self) -> Elf64XWord {
        self.entry_size
    }
}
