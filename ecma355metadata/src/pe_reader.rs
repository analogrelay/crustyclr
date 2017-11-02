use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use format::{CoffHeader, DirectoryType, PeHeader, SectionHeader};
use format::directories::Directory;

use error::Error;

pub struct PeReader<R: Read + Seek> {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    section_headers: Vec<SectionHeader>,
    stream: R,
}

const DOS_SIGNATURE: u16 = 0x5A4D;
const PE_SIGNATURE: u32 = 0x00004550;

impl<R: Read + Seek> PeReader<R> {
    pub fn read(mut stream: R) -> Result<PeReader<R>, Error> {
        // Verify the MZ signature
        let mz_sig = stream.read_u16::<LittleEndian>()?;
        if mz_sig != DOS_SIGNATURE {
            Err(Error::InvalidSignature)
        } else {
            // Seek to the lfanew field
            stream.seek(SeekFrom::Start(0x3C))?;

            // Read the lfanew offset
            let lfanew = stream.read_u32::<LittleEndian>()?;

            // Seek to the PE header
            stream.seek(SeekFrom::Start(lfanew as u64))?;

            // Read the PE signature
            let pe_sig = stream.read_u32::<LittleEndian>()?;

            // Read the COFF header
            let coff_header = CoffHeader::read(&mut stream)?;

            // Read the PE header if there is one
            let pe_header = if pe_sig != PE_SIGNATURE {
                None
            } else {
                Some(PeHeader::read(&mut stream)?)
            };

            // Read section headers
            let section_count = coff_header.number_of_sections as usize;
            let mut section_headers = Vec::with_capacity(section_count);
            for _ in 0..section_count {
                section_headers.push(SectionHeader::read(&mut stream)?);
            }

            // Success!
            Ok(PeReader {
                coff_header: coff_header,
                pe_header: pe_header,
                section_headers: section_headers,
                stream: stream,
            })
        }
    }

    pub fn coff_header(&self) -> &CoffHeader {
        &self.coff_header
    }

    pub fn pe_header(&self) -> Option<&PeHeader> {
        self.pe_header.as_ref()
    }

    pub fn section_headers(&self) -> &Vec<SectionHeader> {
        &self.section_headers
    }
}
