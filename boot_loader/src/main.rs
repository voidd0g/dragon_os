#![no_std]
#![no_main]

use core::{mem::size_of, panic::PanicInfo};
use common::{uefi::{constant::{efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE}, efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, efi_status::{EFI_ABORTED, EFI_SUCCESS}, guid::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID}}, data_types::basic_types::{EFI_HANDLE, EFI_STATUS, UINT32, UINT8, UINTN}, protocols::{efi_file_protocol::EFI_FILE_PROTOCOL, efi_loaded_image_protocol::EFI_LOADED_IMAGE_PROTOCOL, efi_simple_file_system_protocol::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, efi_simple_text_output_protocol::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL}, tables::{efi_boot_services::EFI_BOOT_SERVICES, efi_system_table::EFI_SYSTEM_TABLE}}, utils::from_byte_slice::FromByteSlice};
use utf16_literal::utf16;

#[no_mangle]
pub extern "efiapi" fn efi_main(image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let cout = system_table.con_out();
    let boot_services = system_table.boot_services();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => return v,
    }
    match cout.output_string(utf16!("Hello World\r\n\0")) {
        EFI_SUCCESS => (),
        v => return v,
    }

    // match cout.output_string(utf16!("Get memory map\r\n")) {
    //     EFI_SUCCESS => (),
    //     v => return v,
    // }
    // const MEMMAP_BUF_SIZE: UINTN = 1024;
    // let mut memmap_buf = [0u8; MEMMAP_BUF_SIZE];
    // let mut memmap_size = MEMMAP_BUF_SIZE;
    // let (status, map_key, descriptor_size, descriptor_version) = boot_services.get_memory_map(&mut memmap_size, &mut memmap_buf);
    // match status {
    //     EFI_SUCCESS => (),
    //     v => return v,
    // }
    // let mut memmap = MemoryMap { buffer_size: MEMMAP_BUF_SIZE, memory_map_buffer: &mut memmap_buf, map_size: memmap_size, map_key, descriptor_size, descriptor_version };

    match cout.output_string(utf16!("Open root dir\r\n\0")) {
        EFI_SUCCESS => (),
        v => return v,
    }
    let root_dir = match open_root_dir(image_handle, boot_services) {
        Ok(root_dir) => root_dir,
        Err(v) => return v,
    };

    match cout.output_string(utf16!("Open memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => return v,
    }
    let (status, memmap_file) = root_dir.open(utf16!("memmap\0"), EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE, 0);
    match status {
        EFI_SUCCESS => (),
        v => return v,
    }

    match cout.output_string(utf16!("Close memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => return v,
    }
    match memmap_file.close() {
        EFI_SUCCESS => (),
        v => return v,
    }

    match cout.output_string(utf16!("All done\r\n\0")) {
        EFI_SUCCESS => (),
        v => return v,
    }

    loop{}
}

#[deny(non_snake_case)]
struct MemoryMap<'buffer> {
    buffer_size: UINTN,
    memory_map_buffer: &'buffer [UINT8],
    map_size: UINTN,
    map_key: UINTN, 
    descriptor_size: UINTN, 
    descriptor_version: UINT32,
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

fn save_memory_map(memmap: &MemoryMap, dest_file: &EFI_FILE_PROTOCOL, cout: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS {
    let header = "Index, Type, Type(name), PhysicalStart, NumberOfPages, Attribute\n".as_bytes();
    let mut header_len = header.len();
    match dest_file.write(&mut header_len, header) {
        EFI_SUCCESS => (),
        v => return v,
    }
    match cout.output_string(utf16!("map->buffer = \r\n")) {
        EFI_SUCCESS => (),
        v => return v,
    }
    EFI_SUCCESS
}

fn open_root_dir(image_handle: EFI_HANDLE, boot_services: &EFI_BOOT_SERVICES) -> Result<&EFI_FILE_PROTOCOL, EFI_STATUS> {
    let (status, loaded_image) = boot_services.open_protocol::<EFI_LOADED_IMAGE_PROTOCOL>(image_handle, &EFI_LOADED_IMAGE_PROTOCOL_GUID, Some(()), image_handle, image_handle, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let loaded_image = match loaded_image {
        Some(loaded_image) => loaded_image,
        None => return Err(EFI_ABORTED),
    };

    let (status, fs) = boot_services.open_protocol::<EFI_SIMPLE_FILE_SYSTEM_PROTOCOL>(loaded_image.device_handle(), &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID, Some(()), image_handle, image_handle, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let fs = match fs {
        Some(fs) => fs,
        None => return Err(EFI_ABORTED),
    };

    let (status, root) = fs.open_volume();
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }

    Ok(root)
}