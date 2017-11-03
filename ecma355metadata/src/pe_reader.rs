use std::io::{Read, Seek, SeekFrom};

use byteorder::{LittleEndian, ReadBytesExt};

use format::{CoffHeader, PeHeader, SectionHeader};

use error::Error;

/// Designed for reading a Portable Executable containing ECMA 335 metadata (.NET Assemblies) and parsing data structures
/// 
/// This type is not optimized for loading an ECMA 335 assembly for execution.
pub struct PeReader<R: Read + Seek> {
    coff_header: CoffHeader,
    pe_header: Option<PeHeader>,
    section_headers: Vec<SectionHeader>,
    rva_position: u32,
    stream: R,
}

const DOS_SIGNATURE: u16 = 0x5A4D;
const PE_SIGNATURE: u32 = 0x00004550;

impl<R: Read + Seek> PeReader<R> {
    pub fn new(mut stream: R) -> Result<PeReader<R>, Error> {
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
                rva_position: 0,
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

    pub fn seek_section(&mut self, name: &str) -> Result<(), Error> {
        let section = self.section_headers
            .iter()
            .find(|x| x.name == name)
            .ok_or(Error::SectionNotFound)?;

        // Seek the file to the start of raw data for this section
        self.stream.seek(SeekFrom::Start(section.pointer_to_raw_data as u64))?;
        
        // Set our rva_position
        self.rva_position = section.virtual_address;

        // Success!
        Ok(())
    }

    pub fn seek_rva(&mut self, rva: u32) -> Result<(), Error> {
        let section = self.section_headers
            .iter()
            .find(|x| x.contains_rva(rva))
            .ok_or(Error::SectionNotFound)?;

        // Calculate offset within the section
        let section_offset = rva - section.virtual_address;

        // Clamp it to the size_of_real_data value to avoid seeking past EOF
        let section_offset = if section_offset > section.size_of_raw_data {
            section.size_of_raw_data
        } else {
            section_offset
        };

        // Calulate the file offset matching the section offset
        let file_offset = section.pointer_to_raw_data + section_offset;

        // Seek the file the calculated offset and set rva_position
        self.stream.seek(SeekFrom::Start(file_offset as u64))?;
        self.rva_position = rva;

        // Success!
        Ok(())
    }

    pub fn read_section(&mut self, name: &str, buf: &mut Vec<u8>) -> Result<(), Error> {
        self.seek_section(name)?;

        let read_size = self.current_section().ok_or(Error::SectionNotFound)?.virtual_size as usize;

        // Safety: This reserves the exact amount of space we need, then sets the length
        // to it. The vector now includes uninitialized space, but we're about to fill it
        unsafe {
            buf.reserve_exact(read_size);
            buf.set_len(read_size);
        }

        self.read_exact(&mut buf[0..read_size])?;
        Ok(())
    }

    fn current_section(&self) -> Option<&SectionHeader> {
        self.section_headers.iter().find(|s| s.contains_rva(self.rva_position))
    }
}

impl<R: Read + Seek> Read for PeReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> ::std::io::Result<usize> {
        use std::io;

        // Get the current section
        let (virtual_address, virtual_size, size_of_raw_data) = {
            let s = self.current_section()
                .ok_or(io::Error::new(io::ErrorKind::NotFound, "Not currently in a section"))?;
            (s.virtual_address, s.virtual_size, s.size_of_raw_data)
        };

        let section_offset = self.rva_position as usize - virtual_address as usize;
        let remaining_in_section = virtual_size as usize - section_offset;

        // Determine the maximum amount of data that can be read in the section
        let read_size = if buf.len() > remaining_in_section {
            remaining_in_section
        } else {
            buf.len()
        };

        if read_size == 0 {
            Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Reached end of section, seek to a new section to continue reading"))
        } else {
            let remaining_data_in_section = size_of_raw_data as usize - section_offset;

            // Now, constrain by the size of data to see if we need to fill with zeros
            let physical_read_size = if read_size > remaining_data_in_section {
                remaining_data_in_section
            } else {
                read_size
            };

            // Do the physical read (and error if it fails) 
            if physical_read_size > 0 {
                self.stream.read_exact(&mut buf[0..physical_read_size])?;
            }

            // Fill any remaining amount with zeros
            if physical_read_size < read_size {
                for idx in physical_read_size..read_size {
                    buf[idx as usize] = 0;
                }
            }

            // Return the read amount
            Ok(read_size)
        }
    }
}