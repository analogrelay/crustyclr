use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

#[derive(Debug)]
pub struct DirectoryEntry {
    pub rva: u32,
    pub size: u32
}

impl DirectoryEntry {
    pub fn new(rva: u32, size: u32) -> DirectoryEntry {
        DirectoryEntry {
            rva: rva,
            size: size,
        }
    }

    pub fn read<A: Read>(buf: &mut A) -> Result<DirectoryEntry, Error> {
        Ok(DirectoryEntry {
            rva: buf.read_u32::<LittleEndian>()?,
            size: buf.read_u32::<LittleEndian>()?,
        })
    }
}

impl ::std::fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "0x{:04X} (Size: 0x{:04X})", self.rva, self.size)
    }
}

