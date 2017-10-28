use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use format::{CoffHeader, PeHeader, SectionHeader};

use error::Error;

pub struct PortableExecutable {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    section_headers: Vec<SectionHeader>,
}

const DOS_SIGNATURE: u16 = 0x5A4D;
const PE_SIGNATURE: u32 = 0x00004550;

impl PortableExecutable {
    pub fn read<R: Read + Seek>(r: &mut R) -> Result<PortableExecutable, Error> {
        // Verify the MZ signature
        let mz_sig = r.read_u16::<LittleEndian>()?;
        if mz_sig != DOS_SIGNATURE {
            Err(Error::InvalidSignature)
        } else {
            // Seek to the lfanew field
            r.seek(SeekFrom::Start(0x3C))?;

            // Read the lfanew offset
            let lfanew = r.read_u32::<LittleEndian>()?;

            // Seek to the PE header
            r.seek(SeekFrom::Start(lfanew as u64))?;

            // Read the PE signature
            let pe_sig = r.read_u32::<LittleEndian>()?;
            if pe_sig != PE_SIGNATURE {
                Err(Error::InvalidSignature)
            } else {
                // Read the COFF header
                let coff_header = CoffHeader::read(r)?;

                // Read the PE header next
                let pe_header = PeHeader::read(r)?;

                // Read section headers
                let section_count = coff_header.number_of_sections as usize;
                let mut section_headers = Vec::with_capacity(section_count);
                for _ in 0..section_count {
                    section_headers.push(SectionHeader::read(r)?);
                }

                // Success!
                Ok(PortableExecutable {
                    coff_header: coff_header,
                    pe_header: Some(pe_header),
                    section_headers: section_headers
                })
            }
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
