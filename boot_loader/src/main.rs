#![no_std]
#![no_main]

use core::{mem::size_of, panic::PanicInfo};
use common::{uefi::{constant::{efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE}, efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, efi_status::EFI_SUCCESS, guid::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID}}, data_types::basic_types::{EFI_HANDLE, EFI_STATUS, UINT32, UINT8, UINTN}, protocols::{efi_file_protocol::EFI_FILE_PROTOCOL, efi_loaded_image_protocol::EFI_LOADED_IMAGE_PROTOCOL, efi_simple_file_system_protocol::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL}, tables::{efi_boot_services::EFI_BOOT_SERVICES, efi_system_table::EFI_SYSTEM_TABLE}}, utils::from_byte_slice::FromByteSlice};
use utf16_literal::utf16;

#[no_mangle]
pub extern "C" fn efi_main(image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
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
    match boot_services.get_memory_map(&mut memmap.map_size, memmap.memory_map_buffer, &mut memmap.map_key, &mut memmap.descriptor_size, &mut memmap.descriptor_version) {
        EFI_SUCCESS => (),
        v => return v,
    }

    let root_dir = match open_root_dir(image_handle, boot_services) {
        Ok(root_dir) => root_dir,
        Err(v) => return v,
    };

    root_dir.open(&mut root_dir, utf16!("memmap"), EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE, 0);

    loop{}
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop{}
}

fn open_root_dir(image_handle: EFI_HANDLE, boot_services: &EFI_BOOT_SERVICES) -> Result<EFI_FILE_PROTOCOL, EFI_STATUS> {
    let mut loaded_image_buffer = [0u8; size_of::<EFI_LOADED_IMAGE_PROTOCOL>()];
    match boot_services.open_protocol(image_handle, &EFI_LOADED_IMAGE_PROTOCOL_GUID, Some(&mut loaded_image_buffer), image_handle, image_handle, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL) {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let (loaded_image, _) = EFI_LOADED_IMAGE_PROTOCOL::from_byte_slice(&loaded_image_buffer);

    let mut fs_buffer = [0u8; size_of::<EFI_FILE_PROTOCOL>()];
    match boot_services.open_protocol(loaded_image.device_handle(), &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID, Some(&mut fs_buffer), image_handle, image_handle, EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL) {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let (fs, _) = EFI_SIMPLE_FILE_SYSTEM_PROTOCOL::from_byte_slice(&fs_buffer);

    let mut root_buffer = [0u8; size_of::<EFI_FILE_PROTOCOL>()];
    match fs.open_volume(&mut root_buffer) {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let (root, _) = EFI_FILE_PROTOCOL::from_byte_slice(&root_buffer);

    Ok(root)
}

struct MemoryMap<'buffer> {
    buffer_size: UINTN,
    memory_map_buffer: &'buffer mut [UINT8],
    map_size: UINTN,
    map_key: UINTN, 
    descriptor_size: UINTN, 
    descriptor_version: UINT32,
}