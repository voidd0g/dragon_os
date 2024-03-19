use common::{
    memory_map::MemoryMap,
    uefi::{
        constant::efi_memory_type::{
            EFI_BOOT_SERVICES_CODE, EFI_BOOT_SERVICES_DATA, EFI_CONVENTIONAL_MEMORY,
        },
        data_type::efi_memory_descriptor::UEFI_PAGE_FRAME_SIZE,
    },
};

const MEMORY_MANAGER_MAX_SIZE: usize = 0x1000000000;
const PAGE_FRAME_SIZE: usize = 0x1000;

pub struct BitmapMemoryManager {
    bitmap: [u64; MEMORY_MANAGER_MAX_SIZE / PAGE_FRAME_SIZE / u64::BITS as usize],
}

impl BitmapMemoryManager {
    pub fn new(memory_map: &MemoryMap) -> Self {
        let mut uefi_bitmap =
            [0u64; MEMORY_MANAGER_MAX_SIZE / UEFI_PAGE_FRAME_SIZE / u64::BITS as usize];
        const AVAILABLE_MEMORY_TYPE: [u32; 3] = [
            EFI_BOOT_SERVICES_CODE,
            EFI_BOOT_SERVICES_DATA,
            EFI_CONVENTIONAL_MEMORY,
        ];
        let mut ret_bitmap = [0u64; MEMORY_MANAGER_MAX_SIZE / PAGE_FRAME_SIZE / u64::BITS as usize];
        let mut i = 0usize;
        'a: loop {
            match memory_map.get_nth(i) {
                Some(descriptor) => {
                    if AVAILABLE_MEMORY_TYPE.contains(&descriptor.r#type()) {
                        let physical_start = descriptor.physical_start();
                        for bit in physical_start as usize / UEFI_PAGE_FRAME_SIZE
                            ..(physical_start as usize / UEFI_PAGE_FRAME_SIZE
                                + descriptor.number_of_pages() as usize)
                        {
                            if bit / (u64::BITS as usize) < uefi_bitmap.len() {
                                uefi_bitmap[bit / (u64::BITS as usize)] |=
                                    1 << (bit % (u64::BITS as usize))
                            }
                        }
                    }
                }
                None => break 'a (),
            }
            i += 1;
        }
        for i in 0..ret_bitmap.len() * u64::BITS as usize {
            let start_page_uefi = i * PAGE_FRAME_SIZE / UEFI_PAGE_FRAME_SIZE;
            let end_page_uefi = (i * PAGE_FRAME_SIZE + PAGE_FRAME_SIZE - 1) / UEFI_PAGE_FRAME_SIZE;
            let mut avail = true;
            for j in start_page_uefi..(end_page_uefi + 1) {
                if uefi_bitmap[j / u64::BITS as usize] & 1 << (j % u64::BITS as usize) == 0 {
                    avail = false;
                    break;
                }
            }
            if avail {
                ret_bitmap[i / u64::BITS as usize] |= 1 << (i % u64::BITS as usize)
            }
        }
        Self { bitmap: ret_bitmap }
    }

    pub fn try_allocate(&mut self, num_frames: usize) -> Result<AllocatedArea, ()> {
        let mut i = 0;
        let mut found = 0;
        while i < self.bitmap.len() * u64::BITS as usize {
            if self.bitmap[i / u64::BITS as usize] & 1 << (i % u64::BITS as usize) == 0 {
                found = 0;
            } else {
                found += 1;
                if found == num_frames {
                    return Ok(AllocatedArea::new(i - num_frames + 1, num_frames));
                }
            }
            i += 1;
        }
        Err(())
    }

    pub fn free_area(&mut self, area: AllocatedArea) {
        let start = area.start_page_frame();
        for i in start..(start + area.count()) {
            self.bitmap[i / u64::BITS as usize] |= 1 << (i % u64::BITS as usize);
        }
    }
}

pub struct AllocatedArea {
    start_page_frame: usize,
    count: usize,
}

impl AllocatedArea {
    pub const fn new(start_page_frame: usize, count: usize) -> Self {
        Self {
            start_page_frame,
            count,
        }
    }

    pub fn start_page_frame(&self) -> usize {
        self.start_page_frame
    }

    pub fn count(&self) -> usize {
        self.count
    }
}
