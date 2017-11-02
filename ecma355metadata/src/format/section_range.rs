use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

pub struct SectionRange {
    rva: u32,
    size: u32
}

impl SectionRange {
    pub const SIZE: usize = 8;

    pub fn read<A: Read>(buf: &mut A) -> Result<SectionRange, Error> {
        Ok(SectionRange {
            rva: buf.read_u32::<LittleEndian>()?,
            size: buf.read_u32::<LittleEndian>()?,
        })
    }
}

impl ::std::fmt::Display for SectionRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "0x{:08X} (Size: 0x{:08X})", self.rva, self.size)
    }
}
