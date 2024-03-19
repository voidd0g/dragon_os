use super::basic_type::{Elf64Address, Elf64Half, Elf64Word, Elf64XWord};

#[repr(C)]
pub struct Elf64Sym {
    name: Elf64Word,
    info: u8,
    other: u8,
    section_header_index: Elf64Half,
    value: Elf64Address,
    size: Elf64XWord,
}

impl Elf64Sym {
    pub fn value(&self) -> Elf64Address {
        self.value
    }
}
