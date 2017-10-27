use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use format::PeHeader;

use error::Error;

pub struct PortableExecutable {
    header: PeHeader,
}

impl PortableExecutable {
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<PortableExecutable, Error> {
        // Verify the MZ signature
        let mut mz_sig = [0u8; 2];
        r.read(&mut mz_sig)?;
        if mz_sig[0] != 0x4d || mz_sig[1] != 0x5a {
            Err(Error::InvalidSignature)
        } else {
            // Seek to the lfanew field
            r.seek(SeekFrom::Start(0x3C))?;

            // Read the lfanew offset
            let lfanew = r.read_u32::<LittleEndian>()?;

            // Seek to the PE header
            r.seek(SeekFrom::Start(lfanew as u64))?;

            // Read the PE header
            let pe_header = try!(PeHeader::read(r));

            // Success!
            Ok(PortableExecutable { header: pe_header })
        }
    }

    pub fn header(&self) -> &PeHeader {
        &self.header
    }
}
