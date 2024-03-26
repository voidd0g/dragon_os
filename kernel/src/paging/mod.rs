use core::arch::{asm, global_asm};

use self::page_size::PAGE_SIZE_2M;

pub mod page_size;

const PAGE_TABLE_ALIGN: usize = 0x1000;
const PML4_TABLE_ENTRY_COUNT: usize = 0x200;
const PDP_TABLE_ENTRY_COUNT: usize = 0x200;
const PAGE_DIRECTORY_ENTRY_COUNT: usize = 0x200;
const PAGE_DIRECTORY_COUNT: usize = 0x80;

global_asm!(
    r#"
.section .bss
.align {PAGE_TABLE_ALIGN}
PML4_TABLE:
    .space {PML4_TABLE_ENTRY_COUNT} * 8

.align {PAGE_TABLE_ALIGN}
PDP_TABLE:
	.space {PDP_TABLE_ENTRY_COUNT} * 8

.align {PAGE_TABLE_ALIGN}
PAGE_DIRECTORIES:
	.space {PAGE_DIRECTORY_ENTRY_COUNT} * 8 * {PAGE_DIRECTORY_COUNT}
	
.section .text
"#,
    PAGE_TABLE_ALIGN = const { PAGE_TABLE_ALIGN },
    PML4_TABLE_ENTRY_COUNT = const { PML4_TABLE_ENTRY_COUNT },
    PDP_TABLE_ENTRY_COUNT = const { PDP_TABLE_ENTRY_COUNT },
    PAGE_DIRECTORY_ENTRY_COUNT = const { PAGE_DIRECTORY_ENTRY_COUNT },
    PAGE_DIRECTORY_COUNT = const { PAGE_DIRECTORY_COUNT },
);

extern "C" {
    static mut PML4_TABLE: [u64; PML4_TABLE_ENTRY_COUNT];
    static mut PDP_TABLE: [u64; PDP_TABLE_ENTRY_COUNT];
    static mut PAGE_DIRECTORIES: [[u64; PAGE_DIRECTORY_ENTRY_COUNT]; PAGE_DIRECTORY_COUNT];
}

pub fn setup_identity_page_table_2m() {
    unsafe { PML4_TABLE[0] = ((PDP_TABLE.as_ptr() as u64) & 0x0000_000F_FFFF_F000) + 0x0003 }
    for i in 0..PAGE_DIRECTORY_COUNT {
        unsafe {
            PDP_TABLE[i] = ((PAGE_DIRECTORIES[i].as_ptr() as u64) & 0x0000_000F_FFFF_F000) + 0x0003
        }
        for j in 0..PAGE_DIRECTORY_ENTRY_COUNT {
            unsafe {
                PAGE_DIRECTORIES[i][j] = ((((i * PAGE_DIRECTORY_ENTRY_COUNT + j) as u64)
                    * PAGE_SIZE_2M)
                    & 0x0000_000F_FFFF_F000)
                    + 0x0083
            }
        }
    }
    let pml4_table_addr = unsafe { PML4_TABLE.as_ptr() } as u64;
    unsafe { asm!("mov cr3, rax", in("rax") pml4_table_addr) }
}
