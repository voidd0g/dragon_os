use crate::uefi::data_type::{basic_type::Void, efi_memory_descriptor::EfiMemoryDescriptor};

#[repr(C)]
pub struct MemoryMap {
    memory_map_buffer: *const Void,
    map_size: usize,
    map_key: usize,
    descriptor_size: usize,
    descriptor_version: u32,
}

impl MemoryMap {
    pub const fn new(
        memory_map_buffer: *const Void,
        map_size: usize,
        map_key: usize,
        descriptor_size: usize,
        descriptor_version: u32,
    ) -> Self {
        Self {
            memory_map_buffer,
            map_size,
            map_key,
            descriptor_size,
            descriptor_version,
        }
    }

    pub fn get_nth(&self, index: usize) -> Option<&EfiMemoryDescriptor> {
        if index < self.map_size / self.descriptor_size {
            Some(
                unsafe {
                    ((self.memory_map_buffer as usize + index * self.descriptor_size)
                        as *const EfiMemoryDescriptor)
                        .as_ref()
                }
                .unwrap(),
            )
        } else {
            None
        }
    }

    pub fn map_key(&self) -> usize {
        self.map_key
    }
}
