use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

pub struct MetadataStreamHeader {
    pub offset: u32,
    pub size: u32,
    pub name: String,
}


impl MetadataStreamHeader {
    pub fn read<A: Read>(stream: &mut A) -> Result<MetadataStreamHeader, Error> {
        let offset = stream.read_u32::<LittleEndian>()?;
        let size = stream.read_u32::<LittleEndian>()?;

        // Read no more than 32 nul-terminated bytes
        let name_bytes = read_nul_terminated_bytes(stream, 32)?;
        let name = String::from_utf8(name_bytes)?;

        Ok(MetadataStreamHeader {
            offset: offset,
            size: size,
            name: name,
        })
    }
}

fn read_nul_terminated_bytes<A: Read>(stream: &mut A, max: usize) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    let mut buf = [0u8; 4];
    loop {
        stream.read_exact(&mut buf)?;
        for b in buf.iter() {
            if *b == 0 {
                return Ok(bytes);
            }

            bytes.push(*b);

            // Panic if we go over the max
            assert!(bytes.len() <= max);
        }
    }
}