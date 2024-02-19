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
                Char16, EfiHandle, EfiPhysicalAddress, EfiStatus, UnsignedInt16, UnsignedInt32,
                UnsignedInt64, UnsignedInt8, UnsignedIntNative, Void,
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

fn ascii_to_utf16_literal<const N: usize>(ascii: [UnsignedInt8; N]) -> [Char16; N] {
    ascii.map(|v| v as Char16)
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

    macro_rules! output_string {
        ( $($element:expr),* ) => {
            output_string(&mut [$(&mut $element,)*], cout)
        };
    }

    macro_rules! log_and_end_with_error {
        ( $e:expr, $con_out:expr ) => {
            {
                match $e {
                    Ok(res) => res,
                    Err(v) => {
                        end_with_error! {
                            output_string!(b"Error: ".to_iter_str(IterStrFormat::none()), v.to_iter_str(IterStrFormat::new(Some(Radix::Hexadecimal), Some(true), Some(Padding::new(b'0', 8)))))
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
        output_string!(b"Hello World!".to_iter_str(IterStrFormat::none()))
    }

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let memmap = log_and_end_with_error! {
        get_memory_map(boot_services), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let root_dir = log_and_end_with_error! {
        open_root_dir(image_handle, boot_services, &con_out), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let memmap_file = log_and_end_with_error! {
        root_dir.open(
            &ascii_to_utf16_literal(*b"memmap.txt\0"),
            EFI_FILE_MODE_READ | EFI_FILE_MODE_WRITE | EFI_FILE_MODE_CREATE,
            0,
        ), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        save_memory_map(&memmap, memmap_file, &con_out), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        memmap_file.close(), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        boot_services.free_pool(memmap.memory_map_buffer), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let gop = log_and_end_with_error! {
        open_gop(image_handle, boot_services, &con_out), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    const KERNEL_FILE_NAME: &[Char16] = utf16!(".\\KERNEL.ELF\0");
    let kernel_file = log_and_end_with_error! {
        root_dir.open(KERNEL_FILE_NAME, EFI_FILE_MODE_READ, 0), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    const FILE_INFO_BUFFER_SIZE: UnsignedIntNative =
        size_of::<EfiFileInfo>() + size_of::<Char16>() * KERNEL_FILE_NAME.len();
    let mut kernel_file_info_buffer_size = FILE_INFO_BUFFER_SIZE;
    let mut kernel_file_info_buffer = [0; FILE_INFO_BUFFER_SIZE];
    let _ = log_and_end_with_error! {
        kernel_file.get_info(
            &EFI_FILE_INFO_GUID,
            &mut kernel_file_info_buffer_size,
            &mut kernel_file_info_buffer,
        ), con_out
    };
    let kernel_file_info =
        unsafe { (kernel_file_info_buffer.as_ptr() as *const EfiFileInfo).as_ref() }.unwrap();
    let kernel_file_size = kernel_file_info.file_size();

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let kernel_buf = log_and_end_with_error! {
        boot_services.allocate_pool(EFI_LOADER_DATA, kernel_file_size as UnsignedIntNative), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let mut kernel_file_size_in_out = kernel_file_size as UnsignedIntNative;
    let _ = log_and_end_with_error! {
        kernel_file.read(&mut kernel_file_size_in_out, kernel_buf), con_out
    };
    let kernel_elf_header =
        unsafe { (kernel_buf.as_ptr() as *const Elf64Header).as_ref() }.unwrap();
    let (kernel_beg, kernel_end) = calc_load_address_range(kernel_elf_header);

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
        ), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    copy_load_segments(kernel_elf_header);

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let _ = log_and_end_with_error! {
        boot_services.free_pool(&kernel_buf), con_out
    };

    end_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
    }
    let memmap = log_and_end_with_error! {
        get_memory_map(boot_services), con_out
    };

    let _ = log_and_end_with_error! {
        boot_services.exit_boot_services(image_handle, memmap.map_key), con_out
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

fn output_string(
    elements: &mut [&mut dyn Iterator<Item = UnsignedInt8>],
    cout: &EfiSimpleTextOutputProtocol,
) -> Result<(), ()> {
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
                        if buf_index == buf.len() {
                            cout.output_string(&buf);
                            buf.fill(0);
                            buf_index = 0;
                        }
                    }
                    None => break 'b (),
                }
            },
            None => {
                if buf_index != 0 {
                    cout.output_string(&buf[..buf_index]);
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

macro_rules! return_with_error {
    ( $e:expr ) => {{
        match $e {
            Ok(res) => res,
            Err(v) => return Err(v),
        }
    }};
}

fn open_gop<'a>(
    image_handle: EfiHandle,
    boot_services: &'a EfiBootServices,
    con_out: &ConOut,
) -> Result<&'a EfiGraphicsOutputProtocol, EfiStatus> {
    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
            output_string!(b"true".to_iter_str(IterStrFormat::none()))
        };
        return Err(EFI_ABORTED);
    }

    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
                output_string!(b"true".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
    con_out: &ConOut,
) -> Result<&'a EfiFileProtocol, EfiStatus> {
    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
                output_string!(b"true".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
                output_string!(b"true".to_iter_str(IterStrFormat::none()))
            };
            return Err(EFI_ABORTED);
        }
    };

    let _ = return_with_error! {
        output_string!(b"true".to_iter_str(IterStrFormat::none()))
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
    con_out: &ConOut,
) -> Result<(), EfiStatus> {
    let header = utf16!("Index, Type, PhysicalStart, NumberOfPages, Attribute\n");
    let header_buffer =
        unsafe { slice::from_raw_parts(header.as_ptr() as *const UnsignedInt8, header.len() * 2) };
    let mut header_buffer_size = header_buffer.len();
    let _ = return_with_error! {
        con_out.print(ValueWithFormat::String(header), true)
    };
    let _ = return_with_error! {
        file.write(&mut header_buffer_size, header_buffer)
    };

    let mut descriptor_start = 0;
    let mut i = 0;

    let comma_string = return_with_error! {
        con_out.get_writed_buffer(ValueWithFormat::String(utf16!(", ")))
    };
    let new_line_string = return_with_error! {
        con_out.get_writed_buffer(ValueWithFormat::String(utf16!("\r\n")))
    };

    'a: loop {
        if descriptor_start + memmap.descriptor_size > memmap.map_size {
            let _ = return_with_error! {
                con_out.free_writed_buffer(comma_string)
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(new_line_string)
            };
            break 'a Ok(());
        }
        let descriptor = unsafe {
            (memmap.memory_map_buffer[descriptor_start..].as_ptr() as *const EfiMemoryDescriptor)
                .as_ref()
        }
        .unwrap();

        {
            let _ = return_with_error! {
                con_out.print(ValueWithFormat::UnsignedInt16(i, UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::None, UnsignedIntegerBase::Decimal)), false)
            };
            let string = return_with_error! {
                con_out.get_writed_buffer(ValueWithFormat::UnsignedInt16(i, UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::None, UnsignedIntegerBase::Decimal)))
            };
            let mut string_len = string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &string.get_buffer()[..string.get_buffer_size() - 2])
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(string)
            };
        }

        {
            let _ = return_with_error! {
                output_string!(b"false".to_iter_str(IterStrFormat::none()))
            };
            let mut string_len = comma_string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &comma_string.get_buffer()[..comma_string.get_buffer_size() - 2])
            };
        }

        {
            let _ = return_with_error! {
                con_out.print(ValueWithFormat::UnsignedInt32(descriptor.r#type(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)), false)
            };
            let string = return_with_error! {
                con_out.get_writed_buffer(ValueWithFormat::UnsignedInt32(descriptor.r#type(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)))
            };
            let mut string_len = string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &string.get_buffer()[..string.get_buffer_size() - 2])
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(string)
            };
        }

        {
            let _ = return_with_error! {
                output_string!(b"false".to_iter_str(IterStrFormat::none()))
            };
            let mut string_len = comma_string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &comma_string.get_buffer()[..comma_string.get_buffer_size() - 2])
            };
        }

        {
            let _ = return_with_error! {
                con_out.print(ValueWithFormat::UnsignedInt64(descriptor.physical_start(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)), false)
            };
            let string = return_with_error! {
                con_out.get_writed_buffer(ValueWithFormat::UnsignedInt64(descriptor.physical_start(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)))
            };
            let mut string_len = string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &string.get_buffer()[..string.get_buffer_size() - 2])
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(string)
            };
        }

        {
            let _ = return_with_error! {
                output_string!(b"false".to_iter_str(IterStrFormat::none()))
            };
            let mut string_len = comma_string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &comma_string.get_buffer()[..comma_string.get_buffer_size() - 2])
            };
        }

        {
            let _ = return_with_error! {
                con_out.print(ValueWithFormat::UnsignedInt64(descriptor.physical_start(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)), false)
            };
            let string = return_with_error! {
                con_out.get_writed_buffer(ValueWithFormat::UnsignedInt64(descriptor.physical_start(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)))
            };
            let mut string_len = string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &string.get_buffer()[..string.get_buffer_size() - 2])
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(string)
            };
        }

        {
            let _ = return_with_error! {
                output_string!(b"false".to_iter_str(IterStrFormat::none()))
            };
            let mut string_len = comma_string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &comma_string.get_buffer()[..comma_string.get_buffer_size() - 2])
            };
        }

        {
            let _ = return_with_error! {
                con_out.print(ValueWithFormat::UnsignedInt64(descriptor.attribute(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)), false)
            };
            let string = return_with_error! {
                con_out.get_writed_buffer(ValueWithFormat::UnsignedInt64(descriptor.attribute(), UnsignedIntegerFormatter::new(UnsignedIntegerDigitCount::Fixed { count: 16, fill: '0' as Char16 }, UnsignedIntegerBase::Hexadecimal)))
            };
            let mut string_len = string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &string.get_buffer()[..string.get_buffer_size() - 2])
            };
            let _ = return_with_error! {
                con_out.free_writed_buffer(string)
            };
        }

        {
            let _ = return_with_error! {
                output_string!(b"false".to_iter_str(IterStrFormat::none()))
            };
            let mut string_len = new_line_string.get_buffer_size() - 2;
            let _ = return_with_error! {
                file.write(&mut string_len, &new_line_string.get_buffer()[..new_line_string.get_buffer_size() - 2])
            };
        }

        descriptor_start += memmap.descriptor_size;
        i += 1;
    }
}
