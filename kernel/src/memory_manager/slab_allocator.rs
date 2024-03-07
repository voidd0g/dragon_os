use super::{AllocatedArea, PAGE_FRAME_SIZE};

pub struct Heap {
    heap_area: AllocatedArea,
    slab_32_bytes: Slab,
    slab_64_bytes: Slab,
    slab_128_bytes: Slab,
    slab_256_bytes: Slab,
    slab_512_bytes: Slab,
    slab_1024_bytes: Slab,
    slab_2048_bytes: Slab,
    slab_4096_bytes: Slab,
}

impl Heap {
    pub fn new(heap_area: AllocatedArea) -> Self {
        let heap_start_addr = heap_area.start_page_frame() * PAGE_FRAME_SIZE;
        let heap_size = heap_area.count() * PAGE_FRAME_SIZE;
        let slab_size = heap_size / 8;
        Self {
            heap_area,
            slab_32_bytes: Slab::new(heap_start_addr + 0 * slab_size, slab_size, 32),
            slab_64_bytes: Slab::new(heap_start_addr + 1 * slab_size, slab_size, 64),
            slab_128_bytes: Slab::new(heap_start_addr + 2 * slab_size, slab_size, 128),
            slab_256_bytes: Slab::new(heap_start_addr + 3 * slab_size, slab_size, 256),
            slab_512_bytes: Slab::new(heap_start_addr + 4 * slab_size, slab_size, 512),
            slab_1024_bytes: Slab::new(heap_start_addr + 5 * slab_size, slab_size, 1024),
            slab_2048_bytes: Slab::new(heap_start_addr + 6 * slab_size, slab_size, 2048),
            slab_4096_bytes: Slab::new(heap_start_addr + 7 * slab_size, slab_size, 4096),
        }
    }
}

struct Slab {
    block_size: usize,
    free_block_list: FreeBlockList,
}

impl Slab {
    pub fn new(start_addr: usize, slab_size: usize, block_size: usize) -> Self {
        let num_blocks = slab_size / block_size;
        Self {
            block_size,
            free_block_list: FreeBlockList::new(start_addr, block_size, num_blocks),
        }
    }
}

struct FreeBlockList {
    len: usize,
    head: Option<*mut FreeBlock>,
}

impl FreeBlockList {
    fn new(start_addr: usize, block_size: usize, num_blocks: usize) -> Self {
        let mut ret = Self { len: 0, head: None };
        for i in (0..num_blocks).rev() {
            let new_block = (start_addr + i * block_size) as *mut FreeBlock;
            ret.push(new_block);
        }
        ret
    }

    fn push(&mut self, free_block: *mut FreeBlock) {
        unsafe { free_block.as_mut() }.unwrap().next = self.head.take();
        self.len += 1;
        self.head = Some(free_block);
    }

    fn pop(&mut self) -> Option<*mut FreeBlock> {
        if self.len > 0 {
            let ret = self.head.unwrap();
            self.head = unsafe { ret.as_ref() }.unwrap().next;
            Some(ret)
        } else {
            None
        }
    }
}

struct FreeBlock {
    next: Option<*mut FreeBlock>,
}