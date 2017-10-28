
use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

#[repr(C)]
#[derive(Debug)]
pub struct PeHeader {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub code_size: u32,
    pub initialized_data_size: u32,
    pub uninitialized_data_size: u32,
    pub entry_point_rva: u32,
    pub code_base: u32,
    pub data_base: u32,
}

const PE_MAGIC: u16 = 0x010B;
const PE_PLUS_MAGIC: u16 = 0x020B;

impl PeHeader {
    pub fn read<A: Read>(buf: &mut A) -> Result<PeHeader, Error> {
        // Check the magic number
        let magic = buf.read_u16::<LittleEndian>()?;
        if magic != PE_MAGIC && magic != PE_PLUS_MAGIC {
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
                data_base: buf.read_u32::<LittleEndian>()?,
            })
        }
    }
}
