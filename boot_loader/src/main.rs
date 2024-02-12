#![no_std]
#![no_main]

mod uefi;

use core::{panic::PanicInfo, slice};
use uefi::{constant::{efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE}, efi_memory_type::EfiLoaderData, efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL, efi_status::{EFI_ABORTED, EFI_BAD_BUFFER_SIZE, EFI_BUFFER_TOO_SMALL, EFI_INVALID_PARAMETER, EFI_LOAD_ERROR, EFI_SUCCESS}, guid::{EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID}}, data_types::{basic_types::{CHAR16, EFI_HANDLE, EFI_STATUS, UINT32, UINT8, UINTN}, structs::efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR}, protocols::{efi_file_protocol::EFI_FILE_PROTOCOL, efi_loaded_image_protocol::EFI_LOADED_IMAGE_PROTOCOL, efi_simple_file_system_protocol::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, efi_simple_text_output_protocol::EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL}, tables::{efi_boot_services::EFI_BOOT_SERVICES, efi_system_table::EFI_SYSTEM_TABLE}};
use utf16_literal::utf16;

#[no_mangle]
pub extern "efiapi" fn efi_main(image_handle: EFI_HANDLE, system_table: &EFI_SYSTEM_TABLE) -> EFI_STATUS {
    let cout = system_table.con_out();
    let boot_services = system_table.boot_services();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    match cout.output_string(utf16!("Hello World\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    match cout.output_string(utf16!("Get memmap size\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let mut empty_buf = [];
    let mut memmap_size_needed = 0;
    let (status, _, _, _) = boot_services.get_memory_map(&mut memmap_size_needed, &mut empty_buf);
    match status {
        EFI_SUCCESS => (),
        EFI_BUFFER_TOO_SMALL => {
            let str = match to_string(memmap_size_needed, boot_services, 16) {
                Ok(str) => str,
                Err(v) => return v,
            };
            let str_with_new_line = match concat_string(&[str, utf16!(" bytes needed.\r\n\0")], boot_services) {
                Ok(str) => str,
                Err(v) => return v,
            };
            match cout.output_string(str_with_new_line) {
                EFI_SUCCESS => (),
                v => return v,
            }
            free_string(str_with_new_line, boot_services);
            free_string(str, boot_services);
        },
        v => return error_code(v, boot_services, cout),
    }
    memmap_size_needed += 256;
    memmap_size_needed /= 8;
    memmap_size_needed *= 8;

    match cout.output_string(utf16!("Allocate memmap buffer\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let (status, memmap_buf) = boot_services.allocate_pool(EfiLoaderData, memmap_size_needed);
    match status {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    match cout.output_string(utf16!("Get memory map\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let mut memmap_size = memmap_size_needed;
    let (status, map_key, descriptor_size, descriptor_version) = boot_services.get_memory_map(&mut memmap_size, memmap_buf);
    match status {
        EFI_SUCCESS => (),
        v => _ = error_code(v, boot_services, cout),
    }
    let memmap = MemoryMap { buffer_size: memmap_size_needed, memory_map_buffer: memmap_buf, map_size: memmap_size, map_key, descriptor_size, descriptor_version };

    match cout.output_string(utf16!("Open root dir\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let root_dir = match open_root_dir(image_handle, boot_services) {
        Ok(root_dir) => root_dir,
        Err(v) => return error_code(v, boot_services, cout),
    };

    match cout.output_string(utf16!("Open memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let (status, memmap_file) = root_dir.open(utf16!("memmap.txt\0"), EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE, 0);
    match status {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    

    match cout.output_string(utf16!("Close memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    match memmap_file.close() {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    match cout.output_string(utf16!("Free memmap buffer\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }
    let status = boot_services.free_pool(memmap_buf);
    match status {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    match cout.output_string(utf16!("All done\r\n\0")) {
        EFI_SUCCESS => (),
        v => return error_code(v, boot_services, cout),
    }

    loop{}
}

fn error_code(v: EFI_STATUS, boot_services: &EFI_BOOT_SERVICES, cout: &EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL) -> EFI_STATUS {
    let str = match to_string(v, boot_services, 16) {
        Ok(str) => str,
        Err(v) => return v,
    };
    let str_with_new_line = match concat_string(&[str, utf16!(" : ERROR OCCURED\r\n\0")], boot_services) {
        Ok(str) => str,
        Err(v) => return v,
    };
    match cout.output_string(str_with_new_line) {
        EFI_SUCCESS => (),
        v => return v,
    }
    free_string(str_with_new_line, boot_services);
    free_string(str, boot_services);
    v
}

fn to_string<'a>(efi_status: EFI_STATUS, boot_services: &'a EFI_BOOT_SERVICES, base: u8) -> Result<&'a [CHAR16], EFI_STATUS> {
    let mut mult = efi_status;
    let mut len = 1;
    'b: loop {
        mult /= base as EFI_STATUS;
        len += 1;
        if mult == 0 {
            break 'b ();
        }
    }
    let (status, buf) = boot_services.allocate_pool(EfiLoaderData, len * 2);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let buf = unsafe {
        slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut CHAR16, len)
    };
    let mut iter = buf.iter_mut().rev();
    *iter.next().unwrap() = 0;
    let mut num = efi_status;
    for _ in 0..len - 1 {
        let digit = num % base as EFI_STATUS;
        if digit < 10 {
            *iter.next().unwrap() = ('0' as EFI_STATUS + digit) as CHAR16;
        } else {
            *iter.next().unwrap() = ('A' as EFI_STATUS + digit - 10) as CHAR16;
        }
        num /= base as EFI_STATUS;
    }
    Ok(buf)
}

fn concat_string<'a>(strs: &[&[CHAR16]], boot_services: &'a EFI_BOOT_SERVICES) -> Result<&'a [CHAR16], EFI_STATUS> {
    let len = strs.iter().map(|str| str.len()).sum::<usize>() - strs.len() + 1;
    let (status, buf) = boot_services.allocate_pool(EfiLoaderData, len * 2);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let buf = unsafe {
        slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut CHAR16, len)
    };
    let mut iter = buf.iter_mut();
    for str in strs {
        let mut str_iter = str.iter();
        for _ in 0..str.len() - 1 {
            *iter.next().unwrap() = *str_iter.next().unwrap();
        }
    }
    *iter.next().unwrap() = 0;
    Ok(buf)
}

fn free_string(str: &[CHAR16], boot_services: &EFI_BOOT_SERVICES) -> EFI_STATUS {
    let status = boot_services.free_pool(unsafe {
        slice::from_raw_parts(str.as_ptr() as *const UINT8, str.len() * 2)
    });
    status
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