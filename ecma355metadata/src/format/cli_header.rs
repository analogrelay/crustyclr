use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;
use format::SectionRange;

pub struct CliHeader {
    pub header_size: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata: SectionRange,
    pub flags: u32,
    pub entry_point_token: u32,
    pub resources: SectionRange,
    pub strong_name: SectionRange,
    pub code_manager_table: SectionRange,
    pub vtable_fixups: SectionRange,
    pub export_address_table_jumps: SectionRange,
    pub managed_native_header: SectionRange,
}

impl CliHeader {
    pub const SIZE: usize = 72;

    pub fn read<A: Read>(buf: &mut A) -> Result<CliHeader, Error> {
        Ok(CliHeader {
            header_size: buf.read_u32::<LittleEndian>()?,
            major_runtime_version: buf.read_u16::<LittleEndian>()?,
            minor_runtime_version: buf.read_u16::<LittleEndian>()?,
            metadata: SectionRange::read(buf)?,
            flags: buf.read_u32::<LittleEndian>()?,
            entry_point_token: buf.read_u32::<LittleEndian>()?,
            resources: SectionRange::read(buf)?,
            strong_name: SectionRange::read(buf)?,
            code_manager_table: SectionRange::read(buf)?,
            vtable_fixups: SectionRange::read(buf)?,
            export_address_table_jumps: SectionRange::read(buf)?,
            managed_native_header: SectionRange::read(buf)?,
        })
    }
}