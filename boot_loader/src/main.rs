#![no_std]
#![no_main]

mod to_string;
mod uefi;

use core::{mem::size_of, panic::PanicInfo, ptr::null_mut, slice};
use to_string::ToString;
use uefi::{
    constant::{
        efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE},
        efi_locate_search_type::ByProtocol,
        efi_memory_type::EfiLoaderData,
        efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
        efi_status::{EFI_ABORTED, EFI_BUFFER_TOO_SMALL, EFI_SUCCESS},
        guid::{
            EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, EFI_LOADED_IMAGE_PROTOCOL_GUID,
            EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        },
    },
    data_types::{
        basic_types::{CHAR16, EFI_HANDLE, EFI_STATUS, UINT32, UINT8, UINTN},
        structs::efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR,
    },
    protocols::{
        efi_file_protocol::EFI_FILE_PROTOCOL,
        efi_graphics_output_protocol::EFI_GRAPHICS_OUTPUT_PROTOCOL,
        efi_loaded_image_protocol::EFI_LOADED_IMAGE_PROTOCOL,
        efi_simple_file_system_protocol::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL,
    },
    tables::{efi_boot_services::EFI_BOOT_SERVICES, efi_system_table::EFI_SYSTEM_TABLE},
};
use utf16_literal::utf16;

use crate::uefi::{
    constant::{efi_allocate_type::AllocateAddress, guid::EFI_FILE_INFO_GUID},
    data_types::{basic_types::UINT64, structs::efi_file_info::EFI_FILE_INFO},
};

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: EFI_HANDLE,
    system_table: &EFI_SYSTEM_TABLE,
) -> EFI_STATUS {
    let cout = system_table.con_out();
    let boot_services = system_table.boot_services();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    match cout.output_string(utf16!("Hello World\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Get memory map\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let memmap = match get_memory_map(boot_services) {
        Ok(memmap) => memmap,
        Err(v) => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    };

    match cout.output_string(utf16!("Open root dir\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let root_dir = match open_root_dir(image_handle, boot_services) {
        Ok(root_dir) => root_dir,
        Err(v) => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    };

    match cout.output_string(utf16!("Open memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let (status, memmap_file) = root_dir.open(
        utf16!("memmap.txt\0"),
        EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
        0,
    );
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Save memmap to file\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let status = save_memory_map(&memmap, boot_services, memmap_file);
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Close memmap file\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let status = memmap_file.close();
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Free memmap buffer\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let status = boot_services.free_pool(memmap.memory_map_buffer);
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Get gop\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let gop = match open_gop(image_handle, boot_services) {
        Ok(gop) => gop,
        Err(v) => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    };

    match cout.output_string(utf16!("Draw with gop\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let frame_buffer_mut = unsafe {
        slice::from_raw_parts_mut(
            gop.mode().frame_buffer_base() as *mut [UINT8; 4],
            gop.mode().frame_buffer_size(),
        )
    };
    let mut frame_buffer_mut_iter = frame_buffer_mut.iter_mut();
    let mut i = 0;
    'a: loop {
        match frame_buffer_mut_iter.next() {
            Some(pixel) => *pixel = [if i < 100000 { UINT8::MAX } else { 0 }, if i < 200000 { UINT8::MAX } else { 0 }, if i < 300000 { UINT8::MAX } else { 0 }, 0],
            None => break 'a (),
        }
        i += 1;
        if i == 300000 {
            break 'a ();
        }
    }

    match cout.output_string(utf16!("Open kernel file\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    const KERNEL_FILE_NAME: &[CHAR16] = utf16!(".\\KERNEL.ELF\0");
    let (status, kernel_file) = root_dir.open(KERNEL_FILE_NAME, EFI_FILE_MODE_READ, 0);
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Get kernel file info\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    const FILE_INFO_BUFFER_SIZE: UINTN =
        size_of::<EFI_FILE_INFO>() + size_of::<CHAR16>() * KERNEL_FILE_NAME.len();
    let mut kernel_file_info_buffer_size = FILE_INFO_BUFFER_SIZE;
    let mut kernel_file_info_buffer = [0; FILE_INFO_BUFFER_SIZE];
    let status = kernel_file.get_info(
        &EFI_FILE_INFO_GUID,
        &mut kernel_file_info_buffer_size,
        &mut kernel_file_info_buffer,
    );
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let kernel_file_info =
        unsafe { (kernel_file_info_buffer.as_ptr() as *const EFI_FILE_INFO).as_ref() }.unwrap();
    let kernel_file_size = kernel_file_info.file_size();

    match cout.output_string(utf16!("Allocate page for loading kernel\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let mut kernel_base_addr = 0x72A000;
    const PAGE_SIZE: UINT64 = 0x1000;
    let status = boot_services.allocate_pages(
        AllocateAddress,
        EfiLoaderData,
        ((kernel_file_size + PAGE_SIZE - 1) / PAGE_SIZE) as UINTN,
        &mut kernel_base_addr,
    );
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Load from kernel file\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let mut kernel_file_size_in_out = kernel_file_size as UINTN;
    let status = kernel_file.read(&mut kernel_file_size_in_out, unsafe {
        slice::from_raw_parts_mut(kernel_base_addr as *mut UINT8, kernel_file_size as usize)
    });
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    match cout.output_string(utf16!("Get memory map\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let memmap = match get_memory_map(boot_services) {
        Ok(memmap) => memmap,
        Err(v) => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    };

    match cout.output_string(utf16!("Free memmap buffer\r\n\0")) {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }
    let status = boot_services.free_pool(memmap.memory_map_buffer);
    match status {
        EFI_SUCCESS => (),
        v => {
            let str = (v, 16u8).to_string(boot_services).unwrap();
            cout.output_string(str);
            free_string(str, boot_services);
            end()
        }
    }

    // let status = boot_services.exit_boot_services(image_handle, memmap.map_key);
    // match status {
    //     EFI_SUCCESS => (),
    //     v => {
    //         let str = (v, 16u8).to_string(boot_services).unwrap();
    //         cout.output_string(str);
    //         free_string(str, boot_services);
    //         end()
    //     }
    // }

    // (unsafe { ((kernel_base_addr + 24) as *const extern "sysv64" fn() -> !).read() })();

    end()
}

fn end() -> ! {
    loop {}
}

fn concat_string<'a>(
    strs: &[&[CHAR16]],
    boot_services: &'a EFI_BOOT_SERVICES,
) -> Result<&'a [CHAR16], EFI_STATUS> {
    let len = strs.iter().map(|str| str.len()).sum::<usize>() - strs.len() + 1;
    let (status, buf) = boot_services.allocate_pool(EfiLoaderData, len * 2);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut CHAR16, len) };
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
    let status = boot_services
        .free_pool(unsafe { slice::from_raw_parts(str.as_ptr() as *const UINT8, str.len() * 2) });
    status
}

#[deny(non_snake_case)]
struct MemoryMap<'buffer> {
    memory_map_buffer: &'buffer [UINT8],
    map_size: UINTN,
    map_key: UINTN,
    descriptor_size: UINTN,
    descriptor_version: UINT32,
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

fn open_gop(
    image_handle: EFI_HANDLE,
    boot_services: &EFI_BOOT_SERVICES,
) -> Result<&EFI_GRAPHICS_OUTPUT_PROTOCOL, EFI_STATUS> {
    let (status, num_gop_handles, gop_handles) = boot_services.locate_handle_buffer(
        ByProtocol,
        Some(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID),
        None,
    );
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let (status, gop) = boot_services.open_protocol(
        *gop_handles.iter().nth(0).unwrap(),
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let gop = match gop {
        Some(gop) => gop,
        None => return Err(EFI_ABORTED),
    };

    let status = boot_services.free_pool(unsafe {
        slice::from_raw_parts(
            gop_handles.as_ptr() as *const u8,
            gop_handles.len() * size_of::<EFI_HANDLE>(),
        )
    });
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }

    Ok(gop)
}

fn get_memory_map<'a>(boot_services: &'a EFI_BOOT_SERVICES) -> Result<MemoryMap<'a>, EFI_STATUS> {
    let mut empty_buf = [];
    let mut memmap_size_needed = 0;
    let (status, _, _, _) = boot_services.get_memory_map(&mut memmap_size_needed, &mut empty_buf);
    match status {
        EFI_SUCCESS => (),
        EFI_BUFFER_TOO_SMALL => (),
        v => return Err(v),
    }
    memmap_size_needed += 256;
    memmap_size_needed /= 8;
    memmap_size_needed *= 8;

    let (status, memmap_buf) = boot_services.allocate_pool(EfiLoaderData, memmap_size_needed);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let mut memmap_size = memmap_size_needed;
    let (status, map_key, descriptor_size, descriptor_version) =
        boot_services.get_memory_map(&mut memmap_size, memmap_buf);
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    Ok(MemoryMap {
        memory_map_buffer: memmap_buf,
        map_size: memmap_size,
        map_key,
        descriptor_size,
        descriptor_version,
    })
}

fn open_root_dir(
    image_handle: EFI_HANDLE,
    boot_services: &EFI_BOOT_SERVICES,
) -> Result<&EFI_FILE_PROTOCOL, EFI_STATUS> {
    let (status, loaded_image) = boot_services.open_protocol::<EFI_LOADED_IMAGE_PROTOCOL>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }
    let loaded_image = match loaded_image {
        Some(loaded_image) => loaded_image,
        None => return Err(EFI_ABORTED),
    };

    let (status, fs) = boot_services.open_protocol::<EFI_SIMPLE_FILE_SYSTEM_PROTOCOL>(
        loaded_image.device_handle(),
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    );
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

fn save_memory_map(
    memmap: &MemoryMap,
    boot_services: &EFI_BOOT_SERVICES,
    file: &EFI_FILE_PROTOCOL,
) -> EFI_STATUS {
    let header = utf16!("Index, Type, PhysicalStart, NumberOfPages, Attribute\n");
    let header_buffer =
        unsafe { slice::from_raw_parts(header.as_ptr() as *const UINT8, header.len() * 2) };
    let mut header_buffer_size = header_buffer.len();
    let status = file.write(&mut header_buffer_size, header_buffer);
    match status {
        EFI_SUCCESS => (),
        v => return v,
    }

    let mut descriptor_start = 0;
    let mut i = 0;
    'a: loop {
        if descriptor_start + memmap.descriptor_size > memmap.map_size {
            break 'a EFI_SUCCESS;
        }
        let descriptor = unsafe {
            (memmap.memory_map_buffer[descriptor_start..].as_ptr() as *const EFI_MEMORY_DESCRIPTOR)
                .as_ref()
        }
        .unwrap();
        let i_string = match (i, 10u8).to_string(boot_services) {
            Ok(i_string) => i_string,
            Err(v) => return v,
        };
        let type_string = match (descriptor.r#type(), 16u8).to_string(boot_services) {
            Ok(type_string) => type_string,
            Err(v) => return v,
        };
        let physical_start_string =
            match (descriptor.physical_start(), 16u8).to_string(boot_services) {
                Ok(physical_start_string) => physical_start_string,
                Err(v) => return v,
            };
        let number_of_pages_string =
            match (descriptor.number_of_pages(), 16u8).to_string(boot_services) {
                Ok(number_of_pages_string) => number_of_pages_string,
                Err(v) => return v,
            };
        let attribute_string = match (descriptor.attribute(), 16u8).to_string(boot_services) {
            Ok(attribute_string) => attribute_string,
            Err(v) => return v,
        };
        let str = match concat_string(
            &[
                i_string,
                utf16!(", \0"),
                type_string,
                utf16!(", \0"),
                physical_start_string,
                utf16!(", \0"),
                number_of_pages_string,
                utf16!(", \0"),
                attribute_string,
                utf16!("\n\0"),
            ],
            boot_services,
        ) {
            Ok(attribute_string) => attribute_string,
            Err(v) => return v,
        };
        free_string(attribute_string, boot_services);
        free_string(number_of_pages_string, boot_services);
        free_string(physical_start_string, boot_services);
        free_string(type_string, boot_services);
        free_string(i_string, boot_services);
        let content_buffer = unsafe {
            slice::from_raw_parts(
                str[..str.len() - 1].as_ptr() as *const UINT8,
                (str.len() - 1) * 2,
            )
        };
        let mut content_buffer_size = content_buffer.len();
        let status = file.write(&mut content_buffer_size, content_buffer);
        match status {
            EFI_SUCCESS => (),
            v => return v,
        }
        free_string(str, boot_services);
        descriptor_start += memmap.descriptor_size;
        i += 1;
    }
}
