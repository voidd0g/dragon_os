#![no_std]
#![no_main]

mod ascii_to_utf16;

use ascii_to_utf16::ascii_to_utf16;
use common::{
    argument::{Argument, FrameBufferConfig},
    elf::{
        constant::{
            elf64_program_type::{ELF64_PROGRAM_TYPE_DYNAMIC, ELF64_PROGRAM_TYPE_LOAD},
            elf64_section_type::{
                ELF64_SECTION_TYPE_DYNSYM, ELF64_SECTION_TYPE_REL, ELF64_SECTION_TYPE_RELA,
            },
        },
        elf64_header::Elf64Header,
        elf64_program_header::Elf64ProgramHeader,
        elf64_rel::Elf64Rel,
        elf64_rela::Elf64Rela,
        elf64_section_header::Elf64SectionHeader,
        elf64_sym::Elf64Sym,
    },
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
    memory_map::MemoryMap,
    uefi::{
        constant::{
            efi_allocate_type::AllocateAddress,
            efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE},
            efi_locate_search_type::BY_PROTOCOL,
            efi_memory_type::{EFI_CONVENTIONAL_MEMORY, EFI_LOADER_DATA},
            efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            efi_status::{EFI_ABORTED, EFI_BUFFER_TOO_SMALL},
            guid::{
                EFI_FILE_INFO_GUID, EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
                EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            },
        },
        data_type::{
            basic_type::{EfiHandle, EfiPhysicalAddress, EfiStatus, Void},
            efi_file_info::EfiFileInfo,
        },
        protocol::{
            efi_file_protocol::EfiFileProtocol,
            efi_graphics_output_protocol::EfiGraphicsOutputProtocol,
            efi_loaded_image_protocol::EfiLoadedImageProtocol,
            efi_simple_file_system_protocol::EfiSimpleFileSystemProtocol,
            efi_simple_text_output_protocol::EfiSimpleTextOutputProtocol,
        },
        table::{efi_boot_services::EfiBootServices, efi_system_table::EfiSystemTable},
    },
};
use core::{
    arch::asm,
    mem::{size_of, transmute},
    panic::PanicInfo,
    ptr::{copy, slice_from_raw_parts, write_bytes},
    slice,
};

fn ascii_to_utf16_literal<const N: usize>(ascii: &[u8; N]) -> [u16; N] {
    ascii.map(|v| v as u16)
}

macro_rules! output_string_cout {
    ( $cout:ident, [$( $x:expr ),*] ) => {
        output_string($cout, &mut [
            $(
                &mut $x,
            )*
        ])
    };
    ( $cout:ident, [$( $x:expr, )*] ) => {
        output_string($cout, &mut [
            $(
                &mut $x,
            )*
        ])
    };
}
macro_rules! output_string_file {
    ( $file:ident, [$( $x:expr ),*] ) => {
        output_string_file($file, &mut [
            $(
                &mut $x,
            )*
        ])
    };
    ( $file:ident, [$( $x:expr, )*] ) => {
        output_string_file($file, &mut [
            $(
                &mut $x,
            )*
        ])
    };
}

#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: EfiHandle,
    system_table: *const EfiSystemTable,
) -> EfiStatus {
    let system_table = unsafe { system_table.as_ref() }.unwrap();
    let cout = system_table.con_out();
    let boot_services = system_table.boot_services();

    let _ = match cout.reset(false) {
        Ok(res) => res,
        Err(_) => end(),
    };

    let _ = match output_string_cout!(
        cout,
        [b"Hello World!.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get memory map.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let memmap = match get_memory_map(boot_services) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open root dir.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let root_dir = match open_root_dir(image_handle, boot_services, cout) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open memmap file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    const MEMMAP_FILE_NAME: &[u8; 11] = b"memmap.txt\0";
    let memmap_file = match root_dir.open(
        &ascii_to_utf16_literal(MEMMAP_FILE_NAME),
        EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
        0,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get memmap file info.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    const MEMMAP_FILE_INFO_BUFFER_SIZE: usize =
        size_of::<EfiFileInfo>() + size_of::<u16>() * MEMMAP_FILE_NAME.len();
    let mut memmap_file_info_buffer_size = MEMMAP_FILE_INFO_BUFFER_SIZE;
    let mut memmap_file_info_buffer = [0; MEMMAP_FILE_INFO_BUFFER_SIZE];
    let _ = match memmap_file.get_info(
        &EFI_FILE_INFO_GUID,
        &mut memmap_file_info_buffer_size,
        &mut memmap_file_info_buffer,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };
    let memmap_file_info =
        unsafe { (memmap_file_info_buffer.as_ptr() as *mut EfiFileInfo).as_mut() }.unwrap();

    let _ = match output_string_cout!(
        cout,
        [b"Save memmap to file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match save_memory_map(&memmap, memmap_file, cout) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Close memmap file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match memmap_file.close() {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(cout, [b"Get gop.\r\n".to_iter_str(IterStrFormat::none())]) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let gop = match open_gop(image_handle, boot_services, cout) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open kernel file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    const KERNEL_FILE_NAME: &[u8; 11] = b"KERNEL.ELF\0";
    let kernel_file = match root_dir.open(
        &ascii_to_utf16_literal(KERNEL_FILE_NAME),
        EFI_FILE_MODE_READ,
        0,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get kernel file info.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    const KERNEL_FILE_INFO_BUFFER_SIZE: usize =
        size_of::<EfiFileInfo>() + size_of::<u16>() * KERNEL_FILE_NAME.len();
    let mut kernel_file_info_buffer_size = KERNEL_FILE_INFO_BUFFER_SIZE;
    let mut kernel_file_info_buffer = [0; KERNEL_FILE_INFO_BUFFER_SIZE];
    let _ = match kernel_file.get_info(
        &EFI_FILE_INFO_GUID,
        &mut kernel_file_info_buffer_size,
        &mut kernel_file_info_buffer,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };
    let kernel_file_info =
        unsafe { (kernel_file_info_buffer.as_ptr() as *const EfiFileInfo).as_ref() }.unwrap();
    let kernel_file_size = kernel_file_info.file_size();

    let _ = match output_string_cout!(
        cout,
        [b"Allocate pool for kernel file content.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let kernel_buf = match boot_services.allocate_pool(EFI_LOADER_DATA, kernel_file_size as usize) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [
            b"Read kernel file content to ".to_iter_str(IterStrFormat::none()),
            (kernel_buf.as_ptr() as usize).to_iter_str(IterStrFormat::new(
                Some(Radix::Hexadecimal),
                Some(true),
                Some(Padding::new(b'0', 16))
            )),
            b".\r\n".to_iter_str(IterStrFormat::none())
        ]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let mut kernel_file_size_in_out = kernel_file_size as usize;
    let _ = match kernel_file.read(&mut kernel_file_size_in_out, kernel_buf) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };
    let kernel_elf_header =
        unsafe { (kernel_buf.as_ptr() as *const Elf64Header).as_ref() }.unwrap();
    let (kernel_beg, kernel_end) = calc_load_address_range(kernel_elf_header, cout);

    let _ = match output_string_cout!(
        cout,
        [b"Allocate pages for loading kernel.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    const PAGE_SIZE: u64 = 0x1000;
    let page_num = ((kernel_end - kernel_beg + PAGE_SIZE - 1) / PAGE_SIZE) as usize;
    let offset = {
        let mut i = 0;
        let mut found = None;
        let mut found_num = None;
        'a: loop {
            match memmap.get_nth(i) {
                Some(memmap) => {
                    let pages = memmap.number_of_pages();
                    if memmap.r#type() == EFI_CONVENTIONAL_MEMORY
                        && pages >= page_num as u64
                        && found_num.unwrap_or(u64::MAX) > pages
                    {
                        found = Some(memmap.physical_start());
                        found_num = Some(pages);
                    }
                }
                None => match found {
                    Some(found) => break 'a found,
                    None => {
                        let _ = match output_string_cout!(
                            cout,
                            [b"No enough space to load kernel.\r\n"
                                .to_iter_str(IterStrFormat::none())]
                        ) {
                            Ok(res) => res,
                            Err(_) => end(),
                        };
                    }
                },
            }
            i += 1;
        }
    };
    let mut kernel_base_addr = kernel_beg + offset;
    let _ = match output_string_cout!(
        cout,
        [
            page_num.to_iter_str(IterStrFormat::none()),
            b" pages will be allocated from ".to_iter_str(IterStrFormat::none()),
            kernel_base_addr.to_iter_str(IterStrFormat::new(
                Some(Radix::Hexadecimal),
                Some(true),
                Some(Padding::new(b'0', 8))
            )),
            b".\r\n".to_iter_str(IterStrFormat::none())
        ]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match boot_services.allocate_pages(
        AllocateAddress,
        EFI_LOADER_DATA,
        page_num,
        &mut kernel_base_addr,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Copy content to pages allocated.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    copy_load_segments(kernel_elf_header, offset as usize);

    let _ = match output_string_cout!(
        cout,
        [b"Free pool for kernel file content.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match boot_services.free_pool(&kernel_buf) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Close kernel file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match kernel_file.close() {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get memory map.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let memmap = match get_memory_map(boot_services) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open root dir.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let root_dir = match open_root_dir(image_handle, boot_services, cout) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open memmap file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let memmap_file = match root_dir.open(
        &ascii_to_utf16_literal(MEMMAP_FILE_NAME),
        EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
        0,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get memmap file info.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let mut memmap_file_info_buffer_size = MEMMAP_FILE_INFO_BUFFER_SIZE;
    let mut memmap_file_info_buffer = [0; MEMMAP_FILE_INFO_BUFFER_SIZE];
    let _ = match memmap_file.get_info(
        &EFI_FILE_INFO_GUID,
        &mut memmap_file_info_buffer_size,
        &mut memmap_file_info_buffer,
    ) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };
    let memmap_file_info =
        unsafe { (memmap_file_info_buffer.as_ptr() as *mut EfiFileInfo).as_mut() }.unwrap();

    let _ = match output_string_cout!(
        cout,
        [b"Save memmap to file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match save_memory_map(&memmap, memmap_file, cout) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Close memmap file.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let _ = match memmap_file.close() {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Get memory map.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(_) => end(),
    };
    let memmap = match get_memory_map(boot_services) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

    let _ = match boot_services.exit_boot_services(image_handle, memmap.map_key()) {
        Ok(res) => res,
        Err(v) => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Error: ".to_iter_str(IterStrFormat::none()),
                    v.to_iter_str(IterStrFormat::new(
                        Some(Radix::Hexadecimal),
                        Some(true),
                        Some(Padding::new(b'0', 8))
                    ))
                ]
            ) {
                Ok(res) => res,
                Err(_) => end(),
            };
            end()
        }
    };

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
    let arg = Argument::new(
        &frame_buffer_config,
        system_table.runtime_services(),
        &memmap,
    );

    (unsafe {
        transmute::<*const Void, extern "sysv64" fn(*const Argument) -> !>(
            (*((kernel_base_addr + 24) as *const usize) + offset as usize) as *const Void,
        )
    })(&arg)
}

fn output_string(
    cout: &EfiSimpleTextOutputProtocol,
    elements: &mut [&mut dyn Iterator<Item = u8>],
) -> Result<(), EfiStatus> {
    let mut elements_iter = elements.iter_mut();
    let mut buf = [0; 256];
    let mut buf_index;
    buf.fill(0);
    buf_index = 0;
    'a: loop {
        match elements_iter.next() {
            Some(element) => 'b: loop {
                let mut element = ascii_to_utf16(&mut *element);
                match element.next() {
                    Some(c) => {
                        buf[buf_index] = c;
                        buf_index += 1;
                        if buf_index == buf.len() - 1 {
                            match cout.output_string(&buf) {
                                Ok(res) => res,
                                Err(v) => return Err(v),
                            };
                            buf.fill(0);
                            buf_index = 0;
                        }
                    }
                    None => break 'b (),
                }
            },
            None => {
                if buf_index != 0 {
                    match cout.output_string(&buf[..buf_index + 1]) {
                        Ok(res) => res,
                        Err(v) => return Err(v),
                    };
                }
                break 'a Ok(());
            }
        }
    }
}

fn output_string_file(
    file: &EfiFileProtocol,
    elements: &mut [&mut dyn Iterator<Item = u8>],
) -> Result<(), EfiStatus> {
    let mut elements_iter = elements.iter_mut();
    let mut buf = [0; 256];
    let mut buf_index;
    buf.fill(0);
    buf_index = 0;
    'a: loop {
        match elements_iter.next() {
            Some(element) => 'b: loop {
                match element.next() {
                    Some(c) => {
                        buf[buf_index] = c;
                        buf_index += 1;
                        if buf_index == buf.len() {
                            let mut buffer_size = buf.len();
                            match file.write(&mut buffer_size, &buf) {
                                Ok(res) => res,
                                Err(v) => return Err(v),
                            };
                            buf.fill(0);
                            buf_index = 0;
                        }
                    }
                    None => break 'b (),
                }
            },
            None => {
                if buf_index != 0 {
                    let mut buffer_size = buf_index;
                    match file.write(&mut buffer_size, &buf[..buf_index]) {
                        Ok(res) => res,
                        Err(v) => return Err(v),
                    };
                }
                break 'a Ok(());
            }
        }
    }
}

fn end() -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn copy_load_segments(elf_header: &Elf64Header, kernel_offset: usize) -> () {
    let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
        + elf_header.program_header_offset();

    for _ in 0..elf_header.program_header_num() {
        let program_header =
            unsafe { (cur_address as *const Elf64ProgramHeader).as_ref() }.unwrap();

        match program_header.r#type() {
            ELF64_PROGRAM_TYPE_LOAD => {
                let file_size = program_header.file_size();
                let remaining_size = program_header.memory_size() - file_size;
                let file_offset = program_header.offset();
                let virtual_address = program_header.virtual_address() + kernel_offset;

                unsafe {
                    copy(
                        ((elf_header as *const Elf64Header) as EfiPhysicalAddress + file_offset)
                            as *const u8,
                        virtual_address as *mut u8,
                        file_size as usize,
                    );
                    write_bytes(
                        (virtual_address as EfiPhysicalAddress + file_size) as *mut u8,
                        0,
                        remaining_size as usize,
                    )
                }
            }
            _ => (),
        }

        cur_address += elf_header.program_header_element_size() as EfiPhysicalAddress;
    }

    let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
        + elf_header.section_header_offset();
    for _ in 0..elf_header.section_header_num() {
        let section_header =
            unsafe { (cur_address as *const Elf64SectionHeader).as_ref() }.unwrap();

        match section_header.r#type() {
            ELF64_SECTION_TYPE_DYNSYM => {
                let symb_table = unsafe {
                    slice_from_raw_parts(
                        ((elf_header as *const Elf64Header) as EfiPhysicalAddress
                            + section_header.offset()) as *const Elf64Sym,
                        (section_header.size() / section_header.entry_size()) as usize,
                    )
                    .as_ref()
                }
                .unwrap();

                let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
                    + elf_header.section_header_offset();
                for _ in 0..elf_header.section_header_num() {
                    let section_header =
                        unsafe { (cur_address as *const Elf64SectionHeader).as_ref() }.unwrap();

                    match section_header.r#type() {
                        ELF64_SECTION_TYPE_REL => {
                            let rels = unsafe {
                                slice_from_raw_parts(
                                    ((elf_header as *const Elf64Header) as EfiPhysicalAddress
                                        + section_header.offset())
                                        as *const Elf64Rel,
                                    (section_header.size() / section_header.entry_size()) as usize,
                                )
                                .as_ref()
                            }
                            .unwrap();
                            let mut iter = rels.iter();
                            'a: loop {
                                match iter.next() {
                                    Some(rel) => unsafe {
                                        *((rel.offset() + kernel_offset as u64) as *mut u64) = (symb_table
                                            .iter()
                                            .nth(rel.sym() as usize)
                                            .unwrap()
                                            .value()
                                            + kernel_offset)
                                            as u64
                                    },
                                    None => break 'a (),
                                }
                            }
                        }
                        ELF64_SECTION_TYPE_RELA => {
                            let relas = unsafe {
                                slice_from_raw_parts(
                                    ((elf_header as *const Elf64Header) as EfiPhysicalAddress
                                        + section_header.offset())
                                        as *const Elf64Rela,
                                    (section_header.size() / section_header.entry_size()) as usize,
                                )
                                .as_ref()
                            }
                            .unwrap();
                            let mut iter = relas.iter();
                            'a: loop {
                                match iter.next() {
                                    Some(rela) => unsafe {
                                        *((rela.offset() + kernel_offset as u64) as *mut u64) = ((symb_table
                                            .iter()
                                            .nth(rela.sym() as usize)
                                            .unwrap()
                                            .value()
                                            + kernel_offset)
                                            as i64
                                            + rela.addend())
                                            as u64
                                    },
                                    None => break 'a (),
                                }
                            }
                        }
                        _ => (),
                    }

                    cur_address += elf_header.section_header_element_size() as EfiPhysicalAddress;
                }
                break;
            }
            _ => (),
        }

        cur_address += elf_header.section_header_element_size() as EfiPhysicalAddress;
    }
}

fn calc_load_address_range(
    elf_header: &Elf64Header,
    cout: &EfiSimpleTextOutputProtocol,
) -> (EfiPhysicalAddress, EfiPhysicalAddress) {
    let mut beg = EfiPhysicalAddress::MAX;
    let mut end = 0;
    let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
        + elf_header.program_header_offset();

    for _ in 0..elf_header.program_header_num() {
        let program_header =
            unsafe { (cur_address as *const Elf64ProgramHeader).as_ref() }.unwrap();

        let _ = output_string_cout!(
            cout,
            [
                b"type: ".to_iter_str(IterStrFormat::none()),
                program_header.r#type().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    None,
                    None
                )),
                b", vaddr: ".to_iter_str(IterStrFormat::none()),
                program_header
                    .virtual_address()
                    .to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), None, None)),
                b", memsz: ".to_iter_str(IterStrFormat::none()),
                program_header.memory_size().to_iter_str(IterStrFormat::new(
                    Some(Radix::Hexadecimal),
                    None,
                    None
                )),
                b".\r\n".to_iter_str(IterStrFormat::none()),
            ]
        );

        if program_header.r#type() == ELF64_PROGRAM_TYPE_LOAD {
            let virtual_address = program_header.virtual_address() as EfiPhysicalAddress;
            let cur_beg = virtual_address;
            let cur_end = virtual_address + program_header.memory_size();

            beg = if cur_beg < beg { cur_beg } else { beg };
            end = if end < cur_end { cur_end } else { end };
        }

        cur_address += elf_header.program_header_element_size() as EfiPhysicalAddress;
    }

    let _ = output_string_cout!(
        cout,
        [
            b"from: ".to_iter_str(IterStrFormat::none()),
            beg.to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), None, None)),
            b", to: ".to_iter_str(IterStrFormat::none()),
            end.to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), None, None)),
            b".\r\n".to_iter_str(IterStrFormat::none()),
        ]
    );

    (beg, end)
}

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}

fn open_gop<'a>(
    image_handle: EfiHandle,
    boot_services: &'a EfiBootServices,
    cout: &EfiSimpleTextOutputProtocol,
) -> Result<&'a EfiGraphicsOutputProtocol, EfiStatus> {
    let _ = match output_string_cout!(
        cout,
        [b"Get graphics output protocol handles.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let (num_gop_handles, gop_handles) = match boot_services.locate_handle_buffer(
        BY_PROTOCOL,
        Some(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID),
        None,
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    if num_gop_handles < 1 {
        let _ =
            match output_string_cout!(
                cout,
                [b"No graphics output protocol handles found.\r\n"
                    .to_iter_str(IterStrFormat::none())]
            ) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
        return Err(EFI_ABORTED);
    }

    let _ = match output_string_cout!(
        cout,
        [b"Get graphics output protocol with handle[0].\r\n".to_iter_str(IterStrFormat::none())]
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
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let gop = match gop {
        Some(gop) => gop,
        None => {
            let _ = match output_string_cout!(
                cout,
                [
                    b"Failed to get graphics output protocol with handle[0].\r\n"
                        .to_iter_str(IterStrFormat::none())
                ]
            ) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Free graphics output protocol with handles buffer.\r\n"
            .to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let _ = match boot_services.free_pool(unsafe {
        slice::from_raw_parts(
            gop_handles.as_ptr() as *const u8,
            gop_handles.len() * size_of::<EfiHandle>(),
        )
    }) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    Ok(gop)
}

fn get_memory_map<'a>(boot_services: &'a EfiBootServices) -> Result<MemoryMap, EfiStatus> {
    let mut empty_buf = [];
    let mut memmap_size_needed = 0;
    let _ = match boot_services.get_memory_map(&mut memmap_size_needed, &mut empty_buf) {
        Ok(_) => (),
        Err(EFI_BUFFER_TOO_SMALL) => (),
        Err(v) => return Err(v),
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
    Ok(MemoryMap::new(
        memmap_buf.as_ptr() as *const Void,
        memmap_size,
        map_key,
        descriptor_size,
        descriptor_version,
    ))
}

fn open_root_dir<'a>(
    image_handle: EfiHandle,
    boot_services: &'a EfiBootServices,
    cout: &EfiSimpleTextOutputProtocol,
) -> Result<&'a EfiFileProtocol, EfiStatus> {
    let _ = match output_string_cout!(
        cout,
        [b"Open loaded image protocol.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let loaded_image = match boot_services.open_protocol::<EfiLoadedImageProtocol>(
        image_handle,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let loaded_image = match loaded_image {
        Some(loaded_image) => loaded_image,
        None => {
            let _ = match output_string_cout!(
                cout,
                [b"Failed to get loaded image protocol.\r\n".to_iter_str(IterStrFormat::none())]
            ) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open simple file system protocol.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let fs = match boot_services.open_protocol::<EfiSimpleFileSystemProtocol>(
        loaded_image.device_handle(),
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        Some(()),
        image_handle,
        image_handle,
        EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let fs = match fs {
        Some(fs) => fs,
        None => {
            let _ = match output_string_cout!(
                cout,
                [b"Failed to get simple file system protocol.\r\n"
                    .to_iter_str(IterStrFormat::none())]
            ) {
                Ok(res) => res,
                Err(v) => return Err(v),
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = match output_string_cout!(
        cout,
        [b"Open volume and get root directory.\r\n".to_iter_str(IterStrFormat::none())]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let root = match fs.open_volume() {
        Ok(root) => root,
        Err(v) => return Err(v),
    };

    Ok(root)
}

fn save_memory_map(
    memmap: &MemoryMap,
    file: &EfiFileProtocol,
    cout: &EfiSimpleTextOutputProtocol,
) -> Result<(), EfiStatus> {
    let _ = match output_string_cout!(
        cout,
        [
            b"Index,       Type,      PhysicalStart,      NumberOfPages,          Attribute.\r\n"
                .to_iter_str(IterStrFormat::none())
        ]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };
    let _ = match output_string_file!(
        file,
        [
            b"Index,       Type,      PhysicalStart,      NumberOfPages,          Attribute\n"
                .to_iter_str(IterStrFormat::none())
        ]
    ) {
        Ok(res) => res,
        Err(v) => return Err(v),
    };

    let mut i = 0usize;
    'a: loop {
        match memmap.get_nth(i) {
            Some(descriptor) => {
                let _ = match output_string_file!(
                    file,
                    [
                        i.to_iter_str(IterStrFormat::new(
                            Some(Radix::Decimal),
                            Some(false),
                            Some(Padding::new(b' ', 5))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.r#type().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 8))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.physical_start().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.number_of_pages().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.attribute().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b"\n".to_iter_str(IterStrFormat::none())
                    ]
                ) {
                    Ok(res) => res,
                    Err(v) => return Err(v),
                };
                let _ = match output_string_cout!(
                    cout,
                    [
                        i.to_iter_str(IterStrFormat::new(
                            Some(Radix::Decimal),
                            Some(false),
                            Some(Padding::new(b' ', 5))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.r#type().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 8))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.physical_start().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.number_of_pages().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b", ".to_iter_str(IterStrFormat::none()),
                        descriptor.attribute().to_iter_str(IterStrFormat::new(
                            Some(Radix::Hexadecimal),
                            Some(true),
                            Some(Padding::new(b'0', 16))
                        )),
                        b".\r\n".to_iter_str(IterStrFormat::none())
                    ]
                ) {
                    Ok(res) => res,
                    Err(v) => return Err(v),
                };
            }
            None => break 'a Ok(()),
        }
        i += 1;
    }
}
