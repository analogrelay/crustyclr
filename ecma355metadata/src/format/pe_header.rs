use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;
use format::{PeMagic,Subsystem,DirectoryEntry};

#[derive(Debug)]
pub struct PeHeader {
    pub magic: PeMagic,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub code_size: u32,
    pub initialized_data_size: u32,
    pub uninitialized_data_size: u32,
    pub entry_point_rva: u32,
    pub code_base: u32,
    pub data_base: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_os_version: u16,
    pub minor_os_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub checksum: u32,
    pub subsystem: Subsystem,
    pub dll_flags: u16,
    pub stack_reserve_size: u32,
    pub stack_commit_size: u32,
    pub heap_reserve_size: u32,
    pub heap_commit_size: u32,
    pub loader_flags: u32,
    pub number_of_data_directories: u32,
    pub export_table: DirectoryEntry,
    pub import_table: DirectoryEntry,
    pub resource_table: DirectoryEntry,
    pub exception_table: DirectoryEntry,
    pub certificate_table: DirectoryEntry,
    pub base_relocation_table: DirectoryEntry,
    pub debug_data: DirectoryEntry,
    pub copyright_data: DirectoryEntry,
    pub global_ptr_data: DirectoryEntry,
    pub tls_table: DirectoryEntry,
    pub load_config_table: DirectoryEntry,
    pub bound_import: DirectoryEntry,
    pub import_address_table: DirectoryEntry,
    pub delay_import_descriptor: DirectoryEntry,
    pub cli_header: DirectoryEntry,
    pub reserved: DirectoryEntry,
}

impl PeHeader {
    pub const SIZE: usize = 28;

    pub fn read<A: Read>(buf: &mut A) -> Result<PeHeader, Error> {
        // Check the magic number
        let magic = PeMagic::new(buf.read_u16::<LittleEndian>()?);
        if magic != PeMagic::PE32 && magic != PeMagic::PE32PLUS {
            Err(Error::InvalidSignature)
        } else {
            Ok(PeHeader {
                magic: magic,
                major_linker_version: buf.read_u8()?,
                minor_linker_version: buf.read_u8()?,
                code_size: buf.read_u32::<LittleEndian>()?,
                initialized_data_size: buf.read_u32::<LittleEndian>()?,
                uninitialized_data_size: buf.read_u32::<LittleEndian>()?,
                entry_point_rva: buf.read_u32::<LittleEndian>()?,
                code_base: buf.read_u32::<LittleEndian>()?,
                data_base: if magic.is_pe32plus() {
                    0
                } else {
                    buf.read_u32::<LittleEndian>()?
                },
                image_base: if magic.is_pe32plus() {
                    buf.read_u64::<LittleEndian>()?
                } else {
                    buf.read_u32::<LittleEndian>()? as u64
                },
                section_alignment: buf.read_u32::<LittleEndian>()?,
                file_alignment: buf.read_u32::<LittleEndian>()?,
                major_os_version: buf.read_u16::<LittleEndian>()?,
                minor_os_version: buf.read_u16::<LittleEndian>()?,
                major_image_version: buf.read_u16::<LittleEndian>()?,
                minor_image_version: buf.read_u16::<LittleEndian>()?,
                major_subsystem_version: buf.read_u16::<LittleEndian>()?,
                minor_subsystem_version: buf.read_u16::<LittleEndian>()?,
                win32_version: buf.read_u32::<LittleEndian>()?,
                size_of_image: buf.read_u32::<LittleEndian>()?,
                size_of_headers: buf.read_u32::<LittleEndian>()?,
                checksum: buf.read_u32::<LittleEndian>()?,
                subsystem: Subsystem::new(buf.read_u16::<LittleEndian>()?),
                dll_flags: buf.read_u16::<LittleEndian>()?,
                stack_reserve_size: buf.read_u32::<LittleEndian>()?,
                stack_commit_size: buf.read_u32::<LittleEndian>()?,
                heap_reserve_size: buf.read_u32::<LittleEndian>()?,
                heap_commit_size: buf.read_u32::<LittleEndian>()?,
                loader_flags: buf.read_u32::<LittleEndian>()?,
                number_of_data_directories: buf.read_u32::<LittleEndian>()?,
                export_table: DirectoryEntry::read(buf)?,
                import_table: DirectoryEntry::read(buf)?,
                resource_table: DirectoryEntry::read(buf)?,
                exception_table: DirectoryEntry::read(buf)?,
                certificate_table: DirectoryEntry::read(buf)?,
                base_relocation_table: DirectoryEntry::read(buf)?,
                debug_data: DirectoryEntry::read(buf)?,
                copyright_data: DirectoryEntry::read(buf)?,
                global_ptr_data: DirectoryEntry::read(buf)?,
                tls_table: DirectoryEntry::read(buf)?,
                load_config_table: DirectoryEntry::read(buf)?,
                bound_import: DirectoryEntry::read(buf)?,
                import_address_table: DirectoryEntry::read(buf)?,
                delay_import_descriptor: DirectoryEntry::read(buf)?,
                cli_header: DirectoryEntry::read(buf)?,
                reserved: DirectoryEntry::read(buf)?,
            })
        }
    }
}
