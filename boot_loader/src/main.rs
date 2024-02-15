#![no_std]
#![no_main]

mod to_string;

use common::{
    argument::{Argument, FrameBufferConfig},
    elf::{
        constant::elf64_program_type::ELF64_PROGRAM_TYPE_LOAD, elf64_header::Elf64Header,
        elf64_program_header::Elf64ProgramHeader,
    },
    uefi::{
        constant::{
            efi_allocate_type::AllocateAddress,
            efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE},
            efi_locate_search_type::BY_PROTOCOL,
            efi_memory_type::EFI_LOADER_DATA,
            efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            efi_status::{EFI_ABORTED, EFI_BUFFER_TOO_SMALL, EFI_SUCCESS},
            guid::{
                EFI_FILE_INFO_GUID, EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
                EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            },
        },
        data_type::{
            basic_type::{
                Char16, EfiHandle, EfiStatus, Void, EFI_PHYSICAL_ADDRESS, UnsignedInt32, UnsignedInt64, UnsignedInt8, UnsignedIntNative
            },
            efi_file_info::EfiFileInfo,
            efi_memory_descriptor::EFI_MEMORY_DESCRIPTOR,
        },
        protocol::{
            efi_file_protocol::EFI_FILE_PROTOCOL,
            efi_graphics_output_protocol::EFI_GRAPHICS_OUTPUT_PROTOCOL,
            efi_loaded_image_protocol::EfiLoadedImageProtocol,
            efi_simple_file_system_protocol::EFI_SIMPLE_FILE_SYSTEM_PROTOCOL, efi_simple_text_output_protocol::EfiSimpleTextOutputProtocol,
        },
        table::{efi_boot_services::EFI_BOOT_SERVICES, efi_system_table::EfiSystemTable},
    },
};
use core::{
    arch::asm, mem::{size_of, transmute}, panic::PanicInfo, ptr::{copy, write_bytes}, slice
};
use to_string::ToString;
use utf16_literal::utf16;

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: EfiHandle,
    system_table: *const EfiSystemTable,
) -> EfiStatus {
    let system_table = unsafe { system_table.as_ref() }.unwrap();
    let cout = system_table.con_out();
    let boot_services = system_table.boot_services();
    match cout.reset(false) {
        EFI_SUCCESS => (),
        v => end(),
    }
    cout.output_string(utf16!("Hello World\r\n\0"));

    cout.output_string(utf16!("Get memory map\r\n\0"));
    let memmap = match get_memory_map(boot_services) {
        Ok(memmap) => memmap,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Open root dir\r\n\0"));
    let root_dir = match open_root_dir(image_handle, boot_services) {
        Ok(root_dir) => root_dir,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Open memmap file\r\n\0"));
    let memmap_file = match root_dir.open(
        utf16!("memmap.txt\0"),
        EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
        0,
    ) {
        Ok(memmap_file) => memmap_file,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Save memmap to file\r\n\0"));
    let status = save_memory_map(&memmap, boot_services, memmap_file);
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    cout.output_string(utf16!("Close memmap file\r\n\0"));
    let status = memmap_file.close();
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    cout.output_string(utf16!("Free memmap buffer\r\n\0"));
    let status = boot_services.free_pool(memmap.memory_map_buffer);
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    cout.output_string(utf16!("Get gop\r\n\0"));
    let gop = match open_gop(image_handle, boot_services) {
        Ok(gop) => gop,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Open kernel file\r\n\0"));
    const KERNEL_FILE_NAME: &[Char16] = utf16!(".\\KERNEL.ELF\0");
    let kernel_file = match root_dir.open(KERNEL_FILE_NAME, EFI_FILE_MODE_READ, 0) {
        Ok(kernel_file) => kernel_file,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Get kernel file info\r\n\0"));
    const FILE_INFO_BUFFER_SIZE: UnsignedIntNative =
        size_of::<EfiFileInfo>() + size_of::<Char16>() * KERNEL_FILE_NAME.len();
    let mut kernel_file_info_buffer_size = FILE_INFO_BUFFER_SIZE;
    let mut kernel_file_info_buffer = [0; FILE_INFO_BUFFER_SIZE];
    let status = kernel_file.get_info(
        &EFI_FILE_INFO_GUID,
        &mut kernel_file_info_buffer_size,
        &mut kernel_file_info_buffer,
    );
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }
    let kernel_file_info =
        unsafe { (kernel_file_info_buffer.as_ptr() as *const EfiFileInfo).as_ref() }.unwrap();
    let kernel_file_size = kernel_file_info.file_size();

    cout.output_string(utf16!("Allocate pool for kernel file content\r\n\0"));
    let kernel_buf = match boot_services.allocate_pool(EFI_LOADER_DATA, kernel_file_size as UnsignedIntNative) {
        Ok(res) => res,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    cout.output_string(utf16!("Read kernel file content\r\n\0"));
    let mut kernel_file_size_in_out = kernel_file_size as UnsignedIntNative;
    let status = kernel_file.read(&mut kernel_file_size_in_out, kernel_buf);
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }
    let kernel_elf_header =
        unsafe { (kernel_buf.as_ptr() as *const Elf64Header).as_ref() }.unwrap();
    let (kernel_beg, kernel_end) = calc_load_address_range(kernel_elf_header);

    cout.output_string(utf16!("Allocate pages for loading kernel\r\n\0"));
    let page_num = ((kernel_end - kernel_beg + PAGE_SIZE - 1) / PAGE_SIZE) as UnsignedIntNative;
    let mut kernel_base_addr = kernel_beg;
    const PAGE_SIZE: UnsignedInt64 = 0x1000;
    let status = boot_services.allocate_pages(
        AllocateAddress,
        EFI_LOADER_DATA,
        page_num,
        &mut kernel_base_addr,
    );
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    cout.output_string(utf16!("Copy content to pages allocated\r\n\0"));
    copy_load_segments(kernel_elf_header);

    cout.output_string(utf16!("Free pool for kernel file content\r\n\0"));
    let status = boot_services.free_pool(&kernel_buf);
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    cout.output_string(utf16!("Get memory map\r\n\0"));
    let memmap = match get_memory_map(boot_services) {
        Ok(memmap) => memmap,
        Err(v) => stop_with_error(v, boot_services, cout),
    };

    let status = boot_services.exit_boot_services(image_handle, memmap.map_key);
    match status {
        EFI_SUCCESS => (),
        v => stop_with_error(v, boot_services, cout),
    }

    let graphic_mode = gop.mode();
    let graphic_info = graphic_mode.info();
    let frame_buffer_config = FrameBufferConfig::new(
        graphic_mode.frame_buffer_base() as *mut u8,
        graphic_mode.frame_buffer_size(),
        graphic_info.pixels_per_scan_line(),
        graphic_info.horizontal_resolution(),
        graphic_info.vertical_resolution(),
        graphic_info.pixel_format(),
    );
    let arg = Argument::new(&frame_buffer_config);

    (unsafe {
        transmute::<*const Void, extern "sysv64" fn(*const Argument) -> !>(
            (*((kernel_base_addr + 24) as *const u64)) as *const Void,
        )
    })(&arg)
}

fn stop_with_error(
    v: EfiStatus,
    boot_services: &EFI_BOOT_SERVICES,
    cout: &EfiSimpleTextOutputProtocol,
) -> ! {
    let str = (v, 16u8).to_string(boot_services).unwrap();
    cout.output_string(str);
    free_string(str, boot_services);
    end()
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn copy_load_segments(elf_header: &Elf64Header) -> () {
    let mut cur_address = (elf_header as *const Elf64Header) as EFI_PHYSICAL_ADDRESS
        + elf_header.program_header_offset();

    for _ in 0..elf_header.program_header_num() {
        let program_header =
            unsafe { (cur_address as *const Elf64ProgramHeader).as_ref() }.unwrap();

        if program_header.r#type() == ELF64_PROGRAM_TYPE_LOAD {
            let file_size = program_header.file_size();
            let remaining_size = program_header.memory_size() - file_size;
            let file_offset = program_header.offset();
            let virtual_address = program_header.virtual_address();
            
            unsafe {
                copy(((elf_header as *const Elf64Header) as EFI_PHYSICAL_ADDRESS + file_offset) as *const UnsignedInt8, virtual_address as *mut UnsignedInt8, file_size as UnsignedIntNative);
                write_bytes((virtual_address as EFI_PHYSICAL_ADDRESS + file_size) as *mut UnsignedInt8, 0, remaining_size as UnsignedIntNative)
            }
        }

        cur_address += elf_header.program_header_element_size() as EFI_PHYSICAL_ADDRESS;
    }
}

fn calc_load_address_range(
    elf_header: &Elf64Header,
) -> (EFI_PHYSICAL_ADDRESS, EFI_PHYSICAL_ADDRESS) {
    let mut beg = EFI_PHYSICAL_ADDRESS::MAX;
    let mut end = 0;
    let mut cur_address = (elf_header as *const Elf64Header) as EFI_PHYSICAL_ADDRESS
        + elf_header.program_header_offset();

    for _ in 0..elf_header.program_header_num() {
        let program_header =
            unsafe { (cur_address as *const Elf64ProgramHeader).as_ref() }.unwrap();

        if program_header.r#type() == ELF64_PROGRAM_TYPE_LOAD {
            let virtual_address = program_header.virtual_address() as EFI_PHYSICAL_ADDRESS;
            let cur_beg = virtual_address;
            let cur_end = virtual_address + program_header.memory_size();

            beg = if cur_beg < beg { cur_beg } else { beg };
            end = if end < cur_end { cur_end } else { end };
        }

        cur_address += elf_header.program_header_element_size() as EFI_PHYSICAL_ADDRESS;
    }

    (beg, end)
}

fn concat_string<'a>(
    strs: &[&[Char16]],
    boot_services: &'a EFI_BOOT_SERVICES,
) -> Result<&'a [Char16], EfiStatus> {
    let len = strs.iter().map(|str| str.len()).sum::<usize>() - strs.len() + 1;
    let buf = match boot_services.allocate_pool(EFI_LOADER_DATA, len * 2) {
        Ok(buf) => buf,
        Err(v) => return Err(v),
    };
    let buf = unsafe { slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut Char16, len) };
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

fn free_string(str: &[Char16], boot_services: &EFI_BOOT_SERVICES) -> EfiStatus {
    let status = boot_services
        .free_pool(unsafe { slice::from_raw_parts(str.as_ptr() as *const UnsignedInt8, str.len() * 2) });
    status
}

struct MemoryMap<'buffer> {
    memory_map_buffer: &'buffer [UnsignedInt8],
    map_size: UnsignedIntNative,
    map_key: UnsignedIntNative,
    descriptor_size: UnsignedIntNative,
    descriptor_version: UnsignedInt32,
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn open_gop(
    image_handle: EfiHandle,
    boot_services: &EFI_BOOT_SERVICES,
) -> Result<&EFI_GRAPHICS_OUTPUT_PROTOCOL, EfiStatus> {
    let (num_gop_handles, gop_handles) = match boot_services.locate_handle_buffer(
        BY_PROTOCOL,
        Some(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID),
        None,
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let gop = match boot_services.open_protocol(
        *gop_handles.iter().nth(0).unwrap(),
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(gop) => gop,
        Err(v) => return Err(v),
    };
    let gop = match gop {
        Some(gop) => gop,
        None => return Err(EFI_ABORTED),
    };

    let status = boot_services.free_pool(unsafe {
        slice::from_raw_parts(
            gop_handles.as_ptr() as *const u8,
            gop_handles.len() * size_of::<EfiHandle>(),
        )
    });
    match status {
        EFI_SUCCESS => (),
        v => return Err(v),
    }

    Ok(gop)
}

fn get_memory_map<'a>(boot_services: &'a EFI_BOOT_SERVICES) -> Result<MemoryMap<'a>, EfiStatus> {
    let mut empty_buf = [];
    let mut memmap_size_needed = 0;
    let _ = match boot_services.get_memory_map(&mut memmap_size_needed, &mut empty_buf) {
        Ok(_) => (),
        Err(v) => match v {
            EFI_BUFFER_TOO_SMALL => (),
            v => return Err(v),
        },
    };
    memmap_size_needed += 256;
    memmap_size_needed /= 8;
    memmap_size_needed *= 8;

    let memmap_buf = match boot_services.allocate_pool(EFI_LOADER_DATA, memmap_size_needed) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let mut memmap_size = memmap_size_needed;
    let (map_key, descriptor_size, descriptor_version) =
        match boot_services.get_memory_map(&mut memmap_size, memmap_buf) {
            Ok(res) => res,
            Err(v) => return Err(v),
        };
    Ok(MemoryMap {
        memory_map_buffer: memmap_buf,
        map_size: memmap_size,
        map_key,
        descriptor_size,
        descriptor_version,
    })
}

fn open_root_dir(
    image_handle: EfiHandle,
    boot_services: &EFI_BOOT_SERVICES,
) -> Result<&EFI_FILE_PROTOCOL, EfiStatus> {
    let loaded_image = match boot_services.open_protocol::<EfiLoadedImageProtocol>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(loaded_image) => loaded_image,
        Err(v) => return Err(v),
    };
    let loaded_image = match loaded_image {
        Some(loaded_image) => loaded_image,
        None => return Err(EFI_ABORTED),
    };

    let fs = match boot_services.open_protocol::<EFI_SIMPLE_FILE_SYSTEM_PROTOCOL>(
        loaded_image.device_handle(),
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(fs) => fs,
        Err(v) => return Err(v),
    };
    let fs = match fs {
        Some(fs) => fs,
        None => return Err(EFI_ABORTED),
    };

    let root = match fs.open_volume() {
        Ok(root) => root,
        Err(v) => return Err(v),
    };

    Ok(root)
}

fn save_memory_map(
    memmap: &MemoryMap,
    boot_services: &EFI_BOOT_SERVICES,
    file: &EFI_FILE_PROTOCOL,
) -> EfiStatus {
    let header = utf16!("Index, Type, PhysicalStart, NumberOfPages, Attribute\n");
    let header_buffer =
        unsafe { slice::from_raw_parts(header.as_ptr() as *const UnsignedInt8, header.len() * 2) };
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
            Err(v) => {
                free_string(i_string, boot_services);
                return v;
            }
        };
        let physical_start_string =
            match (descriptor.physical_start(), 16u8).to_string(boot_services) {
                Ok(physical_start_string) => physical_start_string,
                Err(v) => {
                    free_string(type_string, boot_services);
                    free_string(i_string, boot_services);
                    return v;
                }
            };
        let number_of_pages_string =
            match (descriptor.number_of_pages(), 16u8).to_string(boot_services) {
                Ok(number_of_pages_string) => number_of_pages_string,
                Err(v) => {
                    free_string(physical_start_string, boot_services);
                    free_string(type_string, boot_services);
                    free_string(i_string, boot_services);
                    return v;
                }
            };
        let attribute_string = match (descriptor.attribute(), 16u8).to_string(boot_services) {
            Ok(attribute_string) => attribute_string,
            Err(v) => {
                free_string(number_of_pages_string, boot_services);
                free_string(physical_start_string, boot_services);
                free_string(type_string, boot_services);
                free_string(i_string, boot_services);
                return v;
            }
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
            Err(v) => {
                free_string(attribute_string, boot_services);
                free_string(number_of_pages_string, boot_services);
                free_string(physical_start_string, boot_services);
                free_string(type_string, boot_services);
                free_string(i_string, boot_services);
                return v;
            }
        };
        free_string(attribute_string, boot_services);
        free_string(number_of_pages_string, boot_services);
        free_string(physical_start_string, boot_services);
        free_string(type_string, boot_services);
        free_string(i_string, boot_services);
        let content_buffer = unsafe {
            slice::from_raw_parts(
                str[..str.len() - 1].as_ptr() as *const UnsignedInt8,
                (str.len() - 1) * 2,
            )
        };
        let mut content_buffer_size = content_buffer.len();
        let status = file.write(&mut content_buffer_size, content_buffer);
        free_string(str, boot_services);
        match status {
            EFI_SUCCESS => (),
            v => return v,
        }
        descriptor_start += memmap.descriptor_size;
        i += 1;
    }
}
