use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

#[repr(C)]
#[derive(Debug)]
pub struct PeHeader {
    machine: u16,
    number_of_sections: u16,
    timestamp: u32,
    symbol_table_addr: u32,
    symbol_count: u32,
    optional_header_size: u16,
    characteristics: u16,
}

impl PeHeader {
    pub const SIZE: usize = 20;

    pub fn read<A: Read>(buf: &mut A) -> Result<PeHeader, Error> {
        let machine = buf.read_u16::<LittleEndian>()?;
        let number_of_sections = buf.read_u16::<LittleEndian>()?;
        let timestamp = buf.read_u32::<LittleEndian>()?;
        let symbol_table_addr = buf.read_u32::<LittleEndian>()?;
        let symbol_count = buf.read_u32::<LittleEndian>()?;
        let optional_header_size = buf.read_u16::<LittleEndian>()?;
        let characteristics = buf.read_u16::<LittleEndian>()?;

        Ok(PeHeader {
            machine: machine,
            number_of_sections: number_of_sections,
            timestamp: timestamp,
            symbol_table_addr: symbol_table_addr,
            symbol_count: symbol_count,
            optional_header_size: optional_header_size,
            characteristics: characteristics,
        })
    }
}
