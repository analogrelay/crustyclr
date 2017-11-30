use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;
use cli::CliFlags;
use pe::MemoryRange;

pub struct CliHeader {
    pub header_size: u32,
    pub major_runtime_version: u16,
    pub minor_runtime_version: u16,
    pub metadata: MemoryRange,
    pub flags: CliFlags,
    pub entry_point_token: u32,
    pub resources: MemoryRange,
    pub strong_name: MemoryRange,
    pub code_manager_table: MemoryRange,
    pub vtable_fixups: MemoryRange,
    pub export_address_table_jumps: MemoryRange,
    pub managed_native_header: MemoryRange,
}

impl CliHeader {
    pub fn read<A: Read>(buf: &mut A) -> Result<CliHeader, Error> {
        Ok(CliHeader {
            header_size: buf.read_u32::<LittleEndian>()?,
            major_runtime_version: buf.read_u16::<LittleEndian>()?,
            minor_runtime_version: buf.read_u16::<LittleEndian>()?,
            metadata: MemoryRange::read(buf)?,
            flags: CliFlags::from_bits_truncate(buf.read_u32::<LittleEndian>()?),
            entry_point_token: buf.read_u32::<LittleEndian>()?,
            resources: MemoryRange::read(buf)?,
            strong_name: MemoryRange::read(buf)?,
            code_manager_table: MemoryRange::read(buf)?,
            vtable_fixups: MemoryRange::read(buf)?,
            export_address_table_jumps: MemoryRange::read(buf)?,
            managed_native_header: MemoryRange::read(buf)?,
        })
    }
}
