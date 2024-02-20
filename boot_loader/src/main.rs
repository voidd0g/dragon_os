#![no_std]
#![no_main]

mod ascii_to_utf16;

use ascii_to_utf16::ascii_to_utf16;
use common::{
    argument::{Argument, FrameBufferConfig},
    elf::{
        constant::elf64_program_type::ELF64_PROGRAM_TYPE_LOAD, elf64_header::Elf64Header,
        elf64_program_header::Elf64ProgramHeader,
    },
    iter_str::{IterStrFormat, Padding, Radix, ToIterStr},
    uefi::{
        constant::{
            efi_allocate_type::AllocateAddress,
            efi_file_mode::{EFI_FILE_MODE_CREATE, EFI_FILE_MODE_READ, EFI_FILE_MODE_WRITE},
            efi_locate_search_type::BY_PROTOCOL,
            efi_memory_type::EFI_LOADER_DATA,
            efi_open_protocol::EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
            efi_status::{EFI_ABORTED, EFI_BUFFER_TOO_SMALL},
            guid::{
                EFI_FILE_INFO_GUID, EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
                EFI_LOADED_IMAGE_PROTOCOL_GUID, EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            },
        },
        data_type::{
            basic_type::{
                Char16, EfiHandle, EfiPhysicalAddress, EfiStatus, UnsignedInt32, UnsignedInt64,
                UnsignedInt8, UnsignedIntNative, Void,
            },
            efi_file_info::EfiFileInfo,
            efi_memory_descriptor::EfiMemoryDescriptor,
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
    ptr::{copy, write_bytes},
    slice,
};

fn ascii_to_utf16_literal<const N: usize>(ascii: &[UnsignedInt8; N]) -> [Char16; N] {
    ascii.map(|v| v as Char16)
}

macro_rules! output_string_cout {
    ( $cout:ident, $( $x:expr ),* ) => {
        output_string($cout, &mut [
            $(
                &mut $x,
            )*
        ])
    };
}
macro_rules! output_string_file {
    ( $file:ident, $( $x:expr ),* ) => {
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

    macro_rules! end_with_error {
        ( $e:expr ) => {{
            let _ = match $e {
                Ok(res) => res,
                Err(_) => end(),
            };
        }};
    }

    macro_rules! log_and_end_with_error {
        ( $e:expr ) => {
            {
                match $e {
                    Ok(res) => res,
                    Err(v) => {
                        end_with_error! {
                            output_string_cout!(
                                cout,
                                b"Error: ".to_iter_str(IterStrFormat::none()),
                                v.to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 8))))
                            )
                        }
                        end()
                    },
                }
            }
        };
    }

    end_with_error! {
        cout.reset(false)
    }

    end_with_error! {
        output_string_cout!(cout, b"Hello World!\r\n".to_iter_str(IterStrFormat::none()))
    }

    end_with_error! {
        output_string_cout!(cout, b"Get memory map\r\n".to_iter_str(IterStrFormat::none()))
    }
    let memmap = log_and_end_with_error! {
        get_memory_map(boot_services)
    };

    end_with_error! {
        output_string_cout!(cout, b"Open root dir\r\n".to_iter_str(IterStrFormat::none()))
    }
    let root_dir = log_and_end_with_error! {
        open_root_dir(image_handle, boot_services, cout)
    };

    end_with_error! {
        output_string_cout!(cout, b"Open memmap file\r\n".to_iter_str(IterStrFormat::none()))
    }
    const MEMMAP_FILE_NAME: &[UnsignedInt8; 11] = b"memmap.txt\0";
    let memmap_file = log_and_end_with_error! {
        root_dir.open(
            &ascii_to_utf16_literal(MEMMAP_FILE_NAME),
            EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
            0,
        )
    };

    end_with_error! {
        output_string_cout!(cout, b"Get memmap file info\r\n".to_iter_str(IterStrFormat::none()))
    }
    const MEMMAP_FILE_INFO_BUFFER_SIZE: UnsignedIntNative =
        size_of::<EfiFileInfo>() + size_of::<Char16>() * MEMMAP_FILE_NAME.len();
    let mut memmap_file_info_buffer_size = MEMMAP_FILE_INFO_BUFFER_SIZE;
    let mut memmap_file_info_buffer = [0; MEMMAP_FILE_INFO_BUFFER_SIZE];
    let _ = log_and_end_with_error! {
        memmap_file.get_info(
            &EFI_FILE_INFO_GUID,
            &mut memmap_file_info_buffer_size,
            &mut memmap_file_info_buffer,
        )
    };
    let memmap_file_info =
        unsafe { (memmap_file_info_buffer.as_ptr() as *mut EfiFileInfo).as_mut() }.unwrap();

    end_with_error! {
        output_string_cout!(cout, b"Save memmap to file\r\n".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        save_memory_map(&memmap, memmap_file, cout)
    };

    end_with_error! {
        output_string_cout!(cout, b"Close memmap file\r\n".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        memmap_file.close()
    };

    end_with_error! {
        output_string_cout!(cout, b"Free memmap buffer\r\n".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        boot_services.free_pool(memmap.memory_map_buffer)
    };

    end_with_error! {
        output_string_cout!(cout, b"Get gop\r\n".to_iter_str(IterStrFormat::none()))
    }
    let gop = log_and_end_with_error! {
        open_gop(image_handle, boot_services, cout)
    };

    end_with_error! {
        output_string_cout!(cout, b"Open kernel file\r\n".to_iter_str(IterStrFormat::none()))
    }
    const KERNEL_FILE_NAME: &[UnsignedInt8; 11] = b"KERNEL.ELF\0";
    let kernel_file = log_and_end_with_error! {
        root_dir.open(&ascii_to_utf16_literal(KERNEL_FILE_NAME), EFI_FILE_MODE_READ, 0)
    };

    end_with_error! {
        output_string_cout!(cout, b"Get kernel file info\r\n".to_iter_str(IterStrFormat::none()))
    }
    const KERNEL_FILE_INFO_BUFFER_SIZE: UnsignedIntNative =
        size_of::<EfiFileInfo>() + size_of::<Char16>() * KERNEL_FILE_NAME.len();
    let mut kernel_file_info_buffer_size = KERNEL_FILE_INFO_BUFFER_SIZE;
    let mut kernel_file_info_buffer = [0; KERNEL_FILE_INFO_BUFFER_SIZE];
    let _ = log_and_end_with_error! {
        kernel_file.get_info(
            &EFI_FILE_INFO_GUID,
            &mut kernel_file_info_buffer_size,
            &mut kernel_file_info_buffer,
        )
    };
    let kernel_file_info =
        unsafe { (kernel_file_info_buffer.as_ptr() as *const EfiFileInfo).as_ref() }.unwrap();
    let kernel_file_size = kernel_file_info.file_size();

    end_with_error! {
        output_string_cout!(cout, b"Allocate pool for kernel file content\r\n".to_iter_str(IterStrFormat::none()))
    }
    let kernel_buf = log_and_end_with_error! {
        boot_services.allocate_pool(EFI_LOADER_DATA, kernel_file_size as UnsignedIntNative)
    };

    end_with_error! {
        output_string_cout!(cout, b"Read kernel file content\r\n".to_iter_str(IterStrFormat::none()))
    }
    let mut kernel_file_size_in_out = kernel_file_size as UnsignedIntNative;
    let _ = log_and_end_with_error! {
        kernel_file.read(&mut kernel_file_size_in_out, kernel_buf)
    };
    let kernel_elf_header =
        unsafe { (kernel_buf.as_ptr() as *const Elf64Header).as_ref() }.unwrap();
    let (kernel_beg, kernel_end) = calc_load_address_range(kernel_elf_header);

    end_with_error! {
        output_string_cout!(cout, b"Allocate pages for loading kernel\r\n".to_iter_str(IterStrFormat::none()))
    }
    let page_num = ((kernel_end - kernel_beg + PAGE_SIZE - 1) / PAGE_SIZE) as UnsignedIntNative;
    let mut kernel_base_addr = kernel_beg;
    const PAGE_SIZE: UnsignedInt64 = 0x1000;
    let _ = log_and_end_with_error! {
        boot_services.allocate_pages(
            AllocateAddress,
            EFI_LOADER_DATA,
            page_num,
            &mut kernel_base_addr,
        )
    };

    end_with_error! {
        output_string_cout!(cout, b"Copy content to pages allocated\r\n".to_iter_str(IterStrFormat::none()))
    }
    copy_load_segments(kernel_elf_header);

    end_with_error! {
        output_string_cout!(cout, b"Free pool for kernel file content\r\n".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        boot_services.free_pool(&kernel_buf)
    };

    end_with_error! {
        output_string_cout!(cout, b"Get memory map\r\n".to_iter_str(IterStrFormat::none()))
    }
    let memmap = log_and_end_with_error! {
        get_memory_map(boot_services)
    };

    let _ = log_and_end_with_error! {
        boot_services.exit_boot_services(image_handle, memmap.map_key)
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
    let arg = Argument::new(&frame_buffer_config);

    (unsafe {
        transmute::<*const Void, extern "sysv64" fn(*const Argument) -> !>(
            (*((kernel_base_addr + 24) as *const UnsignedIntNative)) as *const Void,
        )
    })(&arg)
}

macro_rules! return_with_error {
    ( $e:expr ) => {{
        match $e {
            Ok(res) => res,
            Err(v) => return Err(v),
        }
    }};
}

fn output_string(
    cout: &EfiSimpleTextOutputProtocol,
    elements: &mut [&mut dyn Iterator<Item = UnsignedInt8>],
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
                            return_with_error! {
                                cout.output_string(&buf)
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
                    return_with_error! {
                        cout.output_string(&buf[..buf_index + 1])
                    };
                }
                break 'a Ok(());
            }
        }
    }
}

fn output_string_file(
    file: &EfiFileProtocol,
    elements: &mut [&mut dyn Iterator<Item = UnsignedInt8>],
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
                            return_with_error! {
                                file.write(&mut buffer_size, &buf)
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
                    return_with_error! {
                        file.write(&mut buffer_size, &buf[..buf_index])
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

fn copy_load_segments(elf_header: &Elf64Header) -> () {
    let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
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
                copy(
                    ((elf_header as *const Elf64Header) as EfiPhysicalAddress + file_offset)
                        as *const UnsignedInt8,
                    virtual_address as *mut UnsignedInt8,
                    file_size as UnsignedIntNative,
                );
                write_bytes(
                    (virtual_address as EfiPhysicalAddress + file_size) as *mut UnsignedInt8,
                    0,
                    remaining_size as UnsignedIntNative,
                )
            }
        }

        cur_address += elf_header.program_header_element_size() as EfiPhysicalAddress;
    }
}

fn calc_load_address_range(elf_header: &Elf64Header) -> (EfiPhysicalAddress, EfiPhysicalAddress) {
    let mut beg = EfiPhysicalAddress::MAX;
    let mut end = 0;
    let mut cur_address = (elf_header as *const Elf64Header) as EfiPhysicalAddress
        + elf_header.program_header_offset();

    for _ in 0..elf_header.program_header_num() {
        let program_header =
            unsafe { (cur_address as *const Elf64ProgramHeader).as_ref() }.unwrap();

        if program_header.r#type() == ELF64_PROGRAM_TYPE_LOAD {
            let virtual_address = program_header.virtual_address() as EfiPhysicalAddress;
            let cur_beg = virtual_address;
            let cur_end = virtual_address + program_header.memory_size();

            beg = if cur_beg < beg { cur_beg } else { beg };
            end = if end < cur_end { cur_end } else { end };
        }

        cur_address += elf_header.program_header_element_size() as EfiPhysicalAddress;
    }

    (beg, end)
}

struct MemoryMap<'buffer> {
    memory_map_buffer: &'buffer [UnsignedInt8],
    map_size: UnsignedIntNative,
    map_key: UnsignedIntNative,
    descriptor_size: UnsignedIntNative,
    _descriptor_version: UnsignedInt32,
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
    let _ = return_with_error! {
        output_string_cout!(cout, b"Get graphics output protocol handles\r\n".to_iter_str(IterStrFormat::none()))
    };
    let (num_gop_handles, gop_handles) = return_with_error! {
        boot_services.locate_handle_buffer(
            BY_PROTOCOL,
            Some(&EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID),
            None,
        )
    };
    if num_gop_handles < 1 {
        let _ = return_with_error! {
            output_string_cout!(cout, b"No graphics output protocol handles found\r\n".to_iter_str(IterStrFormat::none()))
        };
        return Err(EFI_ABORTED);
    }

    let _ = return_with_error! {
        output_string_cout!(cout, b"Get graphics output protocol with handle[0]\r\n".to_iter_str(IterStrFormat::none()))
    };
    let gop = return_with_error! {
        boot_services.open_protocol(
            *gop_handles.iter().nth(0).unwrap(),
            &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
            Some(()),
            image_handle,
            image_handle,
            EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
        )
    };
    let gop = match gop {
        Some(gop) => gop,
        None => {
            let _ = return_with_error! {
                output_string_cout!(cout, b"Failed to get graphics output protocol with handle[0]\r\n".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string_cout!(cout, b"Free graphics output protocol with handles buffer\r\n".to_iter_str(IterStrFormat::none()))
    };
    let _ = return_with_error! {
        boot_services.free_pool(unsafe {
            slice::from_raw_parts(
                gop_handles.as_ptr() as *const u8,
                gop_handles.len() * size_of::<EfiHandle>(),
            )
        })
    };
    Ok(gop)
}

fn get_memory_map<'a>(boot_services: &'a EfiBootServices) -> Result<MemoryMap<'a>, EfiStatus> {
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

    let memmap_buf = return_with_error! {
        boot_services.allocate_pool(EFI_LOADER_DATA, memmap_size_needed)
    };
    let mut memmap_size = memmap_size_needed;
    let (map_key, descriptor_size, descriptor_version) = return_with_error! {
        boot_services.get_memory_map(&mut memmap_size, memmap_buf)
    };
    Ok(MemoryMap {
        memory_map_buffer: memmap_buf,
        map_size: memmap_size,
        map_key,
        descriptor_size,
        _descriptor_version: descriptor_version,
    })
}

fn open_root_dir<'a>(
    image_handle: EfiHandle,
    boot_services: &'a EfiBootServices,
    cout: &EfiSimpleTextOutputProtocol,
) -> Result<&'a EfiFileProtocol, EfiStatus> {
    let _ = return_with_error! {
        output_string_cout!(cout, b"Open loaded image protocol\r\n".to_iter_str(IterStrFormat::none()))
    };
    let loaded_image = return_with_error! {
        boot_services.open_protocol::<EfiLoadedImageProtocol>(
            image_handle,
            &EFI_LOADED_IMAGE_PROTOCOL_GUID,
            Some(()),
            image_handle,
            image_handle,
            EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
        )
    };
    let loaded_image = match loaded_image {
        Some(loaded_image) => loaded_image,
        None => {
            let _ = return_with_error! {
                output_string_cout!(cout, b"Failed to get loaded image protocol\r\n".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string_cout!(cout, b"Open simple file system protocol\r\n".to_iter_str(IterStrFormat::none()))
    };
    let fs = return_with_error! {
            boot_services.open_protocol::<EfiSimpleFileSystemProtocol>(
            loaded_image.device_handle(),
            &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
            Some(()),
            image_handle,
            image_handle,
            EFI_OPEN_PROTOCOL_BY_HANDLE_PROTOCOL,
        )
    };
    let fs = match fs {
        Some(fs) => fs,
        None => {
            let _ = return_with_error! {
                output_string_cout!(cout, b"Failed to get simple file system protocol\r\n".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string_cout!(cout, b"Open volume and get root directory\r\n".to_iter_str(IterStrFormat::none()))
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
    let _ = return_with_error! {
        output_string_cout!(cout, b"Index,       Type,      PhysicalStart,      NumberOfPages,          Attribute\r\n".to_iter_str(IterStrFormat::none()))
    };
    let _ = return_with_error! {
        output_string_file!(file, b"Index,       Type,      PhysicalStart,      NumberOfPages,          Attribute\n".to_iter_str(IterStrFormat::none()))
    };

    let mut descriptor_start = 0;
    let mut i = 0usize;

    'a: loop {
        if descriptor_start + memmap.descriptor_size > memmap.map_size {
            break 'a Ok(());
        }
        let descriptor = unsafe {
            (memmap.memory_map_buffer[descriptor_start..].as_ptr() as *const EfiMemoryDescriptor)
                .as_ref()
        }
        .unwrap();

        let _ = return_with_error! {
            output_string_file!(file,
                i.to_iter_str(IterStrFormat::new(Some(Radix::Decimal), Some(false), Some(Padding::new(b' ', 5)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.r#type().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 8)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.physical_start().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.number_of_pages().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.attribute().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b"\n".to_iter_str(IterStrFormat::none())
            )
        };
        let _ = return_with_error! {
            output_string_cout!(cout,
                i.to_iter_str(IterStrFormat::new(Some(Radix::Decimal), Some(false), Some(Padding::new(b' ', 5)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.r#type().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 8)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.physical_start().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.number_of_pages().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b", ".to_iter_str(IterStrFormat::none()),
                descriptor.attribute().to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 16)))),
                b"\r\n".to_iter_str(IterStrFormat::none())
            )
        };

        descriptor_start += memmap.descriptor_size;
        i += 1;
    }
}
