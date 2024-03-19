use super::basic_type::{Elf64Offset, Elf64SXWord, Elf64Word, Elf64XWord};

#[repr(C)]
pub struct Elf64Rela {
    offset: Elf64Offset,
    info: Elf64XWord,
    addend: Elf64SXWord,
}

impl Elf64Rela {
    pub fn offset(&self) -> Elf64Offset {
        self.offset
    }
    pub fn sym(&self) -> Elf64Word {
        (self.info >> 32) as Elf64Word
    }
    pub fn r#type(&self) -> Elf64Word {
        self.info as Elf64Word
    }
    pub fn addend(&self) -> Elf64SXWord {
        self.addend
    }
}
