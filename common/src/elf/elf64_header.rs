use super::{
    basic_type::{
        Elf64Address, Elf64Flag, Elf64Half, Elf64Machine, Elf64Offset, Elf64Type, Elf64Version,
    },
    constant::elf64_ident::ELF64_IDENT_IDENT_LEN,
};

#[repr(C)]
pub struct Elf64Header {
    identifier: [u8; ELF64_IDENT_IDENT_LEN],
    r#type: Elf64Type,
    machine: Elf64Machine,
    version: Elf64Version,
    entry: Elf64Address,
    program_header_offset: Elf64Offset,
    section_header_offset: Elf64Offset,
    flags: Elf64Flag,
    header_size: Elf64Half,
    program_header_element_size: Elf64Half,
    program_header_num: Elf64Half,
    section_header_element_size: Elf64Half,
    section_header_num: Elf64Half,
    section_header_string_index: Elf64Half,
}

impl Elf64Header {
    pub fn program_header_offset(&self) -> Elf64Offset {
        self.program_header_offset
    }
    pub fn program_header_element_size(&self) -> Elf64Half {
        self.program_header_element_size
    }
    pub fn program_header_num(&self) -> Elf64Half {
        self.program_header_num
    }
}
