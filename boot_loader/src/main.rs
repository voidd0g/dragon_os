#![no_std]
#![no_main]

use core::panic::PanicInfo;
use common::uefi::{data_types::{common_types::{EFI_HANDLE, EFI_STATUS, EFI_SUCCESS, UINT32, UINT8, UINTN}, structs::EFI_MEMORY_DESCRIPTOR}, tables::efi_system_table::EFI_SYSTEM_TABLE};
use utf16_literal::utf16;

#[no_mangle]
pub extern "C" fn efi_main(_image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let cout = system_table.con_out();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => return v,
    }
    match cout.output_string(utf16!("Hello World\r\n")) {
        EFI_SUCCESS => (),
        v => return v,
    }

    const MEMMAP_BUF_SIZE: UINTN = 4096 * 4;
    let mut memmap_buf = [0u8; MEMMAP_BUF_SIZE];
    let mut memmap = MemoryMap { buffer_size: MEMMAP_BUF_SIZE, memory_map_buffer: &mut memmap_buf, map_size: MEMMAP_BUF_SIZE, map_key: 0, descriptor_size: 0, descriptor_version: 0 };
    let boot_services = system_table.boot_services();
    match boot_services.get_memory_map(&mut memmap.buffer_size, memmap.memory_map_buffer, &mut memmap.map_key, &mut memmap.descriptor_size, &mut memmap.descriptor_version) {
        EFI_SUCCESS => (),
        v => return v,
    }

    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

struct MemoryMap<'buffer> {
    buffer_size: UINTN,
    memory_map_buffer: &'buffer mut [UINT8],
    map_size: UINTN,
    map_key: UINTN, 
    descriptor_size: UINTN, 
    descriptor_version: UINT32,
}