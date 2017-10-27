use std::io::Read;

use format::Error;

#[repr(C)]
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
    pub fn read<A: AsRef<[u8]>>(buf: A) -> Result<PeHeader> {
        let buf = buf.as_ref();
    }
}
