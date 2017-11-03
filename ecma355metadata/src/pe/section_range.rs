use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

pub struct SectionRange {
    pub rva: u32,
    pub size: u32,
}

impl SectionRange {
    pub const SIZE: usize = 8;

    pub fn new(rva: u32, size: u32) -> SectionRange {
        SectionRange {
            rva: rva,
            size: size,
        }
    }

    pub fn read<A: Read>(buf: &mut A) -> Result<SectionRange, Error> {
        Ok(SectionRange::new(
            buf.read_u32::<LittleEndian>()?,
            buf.read_u32::<LittleEndian>()?,
        ))
    }

    pub fn end(&self) -> u32 {
        self.rva + self.size
    }
}

impl ::std::fmt::Display for SectionRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "0x{:08X} [Size: 0x{:08X}]", self.rva, self.size)
    }
}
