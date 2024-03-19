use super::basic_type::{Elf64Offset, Elf64Word, Elf64XWord};

#[repr(C)]
pub struct Elf64Rel {
    offset: Elf64Offset,
    info: Elf64XWord,
}

impl Elf64Rel {
    pub fn offset(&self) -> Elf64Offset {
        self.offset
    }
    pub fn sym(&self) -> Elf64Word {
        (self.info >> 32) as Elf64Word
    }
    pub fn r#type(&self) -> Elf64Word {
        self.info as Elf64Word
    }
}
