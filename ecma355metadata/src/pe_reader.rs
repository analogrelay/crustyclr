use std::io::{Read, Seek, SeekFrom, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};

use format::{CoffHeader, PeHeader, SectionHeader};

use error::Error;

pub struct PeReader<R: Read + Seek> {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    section_headers: Vec<SectionHeader>,
    section_data: Vec<Option<Vec<u8>>>,
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
            let mut section_data = Vec::with_capacity(section_count);
            for _ in 0..section_count {
                section_headers.push(SectionHeader::read(&mut stream)?);
                section_data.push(None);
            }

            // Success!
            Ok(PeReader {
                coff_header: coff_header,
                pe_header: pe_header,
                section_headers: section_headers,
                section_data: section_data,
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

    pub fn create_reader<'a>(&'a mut self, rva: u32) -> Result<Cursor<&'a [u8]>, Error> {
        // Look up the index of the containing the RVA
        let section_index = match self.section_headers.iter().position(|s| s.contains_rva(rva)) {
            Some(x) => x,
            None => return Err(Error::SectionNotFound),
        };

        let offset = rva - self.section_headers[section_index].virtual_address;

        let buf = self.get_section_by_index(section_index)?;
        let mut cur = Cursor::new(buf);
        cur.seek(SeekFrom::Start(offset as u64));
        Ok(cur)
    }

    pub fn get_section(&mut self, section_name: &str) -> Result<&[u8], Error> {
        // Look up the index of the section
        let section_index = match self.section_headers.iter().position(|s| s.name == section_name) {
            Some(x) => x,
            None => return Err(Error::SectionNotFound),
        };
        self.get_section_by_index(section_index)
    }

    fn get_section_by_index(&mut self, section_index: usize) -> Result<&[u8], Error> {
        if self.section_data[section_index].is_some() {
            let data = self.section_data[section_index].as_ref().unwrap();
            Ok(data.as_slice())
        }
        else {
            let header = &self.section_headers[section_index];

            // Allocate a Vec of the appropriate capacity and then push the length out to that far.
            // This is unsafe because if we didn't do with_capacity we'd be expanding the Vec over
            // uninitialized memory.
            let mut section_buf = unsafe {
                let mut v = Vec::with_capacity(header.size_of_raw_data as usize);
                v.set_len(header.size_of_raw_data as usize);
                v
            };

            // Seek and fill the buffer
            self.stream.seek(SeekFrom::Start(header.pointer_to_raw_data as u64))?;
            self.stream.read_exact(section_buf.as_mut_slice())?;

            // Store this back in the option
            self.section_data[section_index] = Some(section_buf);
            Ok(self.section_data[section_index].as_ref().unwrap().as_slice())
        }
    }
}
