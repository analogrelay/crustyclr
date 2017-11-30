use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use pe::{CoffHeader, DirectoryEntry, DirectoryType, MemoryRange, PeHeader, SectionHeader};
use error::Error;

/// Represents a Portable Executable Image, loaded into memory.
pub struct PeImage {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    sections: Vec<SectionHeader>,
}

const DOS_SIGNATURE: u16 = 0x5A4D;
const PE_SIGNATURE: u32 = 0x00004550;

impl PeImage {
    pub fn read<R: Read + Seek>(mut reader: R) -> Result<PeImage, Error> {
        // Verify the MZ signature
        let mz_sig = reader.read_u16::<LittleEndian>()?;
        if mz_sig != DOS_SIGNATURE {
            Err(Error::InvalidSignature)
        } else {
            // Seek to the lfanew field
            reader.seek(SeekFrom::Start(0x3C))?;

            // Read the lfanew offset
            let lfanew = reader.read_u32::<LittleEndian>()?;

            // Seek to the PE header
            reader.seek(SeekFrom::Start(lfanew as u64))?;

            // Read the PE signature
            let pe_sig = reader.read_u32::<LittleEndian>()?;

            // Read the COFF header
            let coff_header = CoffHeader::read(&mut reader)?;

            // Read the PE header if there is one
            let pe_header = if pe_sig != PE_SIGNATURE {
                None
            } else {
                Some(PeHeader::read(&mut reader)?)
            };

            // Read section headers
            let section_count = coff_header.number_of_sections as usize;
            let mut sections = Vec::with_capacity(section_count);
            for _ in 0..section_count {
                sections.push(SectionHeader::read(&mut reader)?);
            }

            Ok(PeImage {
                coff_header: coff_header,
                pe_header: pe_header,
                sections: sections,
            })
        }
    }

    pub fn coff_header(&self) -> &CoffHeader {
        &self.coff_header
    }

    pub fn pe_header(&self) -> Option<&PeHeader> {
        self.pe_header.as_ref()
    }

    pub fn sections(&self) -> &Vec<SectionHeader> {
        &self.sections
    }

    pub fn get_directory(&self, directory_type: DirectoryType) -> Option<&DirectoryEntry> {
        if let Some(pe_header) = self.pe_header() {
            pe_header
                .directories()
                .iter()
                .find(|d| d.directory_type == directory_type)
        } else {
            None
        }
    }

    pub fn map_virtual_range(&self, range: MemoryRange) -> Option<MemoryRange> {
        self.sections
            .iter()
            .find(|h| h.contains_rva(range.start))
            .map(|h| {
                let offset = range.start - h.virtual_address;
                MemoryRange::new(h.pointer_to_raw_data + offset, h.size_of_raw_data - offset)
            })
    }
}
