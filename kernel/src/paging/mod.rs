use core::{arch::global_asm, ptr::addr_of};

use self::page_size::PAGE_SIZE_2M;

pub mod page_size;

const PAGE_TABLE_ALIGN: usize = 4096;
const PML4_TABLE_ENTRY_COUNT: usize = 0x0200;
const PDP_TABLE_ENTRY_COUNT: usize = 0x0200;
const PAGE_DIRECTORY_ENTRY_COUNT: usize = 0x0200;
const PAGE_DIRECTORY_COUNT: usize = 0x0040;

global_asm!(
    r#"
.extern kernel_main_core

.section .bss
.align {PAGE_TABLE_ALIGN}
PML4_TABLE:
    .space {PML4_TABLE_ENTRY_COUNT} * 64
PDP_TABLE:
	.space {PDP_TABLE_ENTRY_COUNT} * 64
PAGE_DIRECTORIES:
	.space {PAGE_DIRECTORY_ENTRY_COUNT} * 64 * {PAGE_DIRECTORY_COUNT}
	
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
    unsafe { PML4_TABLE[0] = ((addr_of!(PDP_TABLE) as u64) & 0x0000_000F_FFFF_F000) + 0x0003 }
    for i in 0..PAGE_DIRECTORY_COUNT {
        unsafe {
            PDP_TABLE[i] = ((addr_of!(PAGE_DIRECTORIES[i]) as u64) & 0x0000_000F_FFFF_F000) + 0x0003
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
    unsafe { set_cr3(addr_of!(PML4_TABLE) as u64) }
}

extern "C" {
    fn set_cr3(val: u64);
}

global_asm!(
    r#"
set_cr3:
    mov cr3, rdi
    ret
"#
);
