use std::io::{self, Read, Seek, SeekFrom};

use pe::PeSection;

#[derive(Clone)]
pub struct SectionReader<'a> {
    section: &'a PeSection,
    offset: u32,
}

impl<'a> SectionReader<'a> {
    pub fn new(section: &'a PeSection, offset: u32) -> SectionReader {
        SectionReader {
            section: section,
            offset: offset
        }
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }
}

impl<'a> Seek for SectionReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, io::Error> {
        match pos {
            SeekFrom::Start(x) => self.offset = x as u32,
            SeekFrom::Current(x) => self.offset = (self.offset as i64 + x) as u32,
            SeekFrom::End(x) => {
                self.offset = (self.section.header().virtual_end() as i64 - x) as u32
            }
        }

        Ok(self.offset as u64)
    }
}

impl<'a> Read for SectionReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, io::Error> {
        // Clamp the read size to whatever is left
        let remaining_to_read = self.section.header().virtual_size as usize - self.offset as usize;
        let read_size = if buf.len() > remaining_to_read {
            remaining_to_read
        } else {
            buf.len()
        };

        // If there isn't any data to be read, return EOF
        if read_size == 0 {
            Ok(0)
        } else {
            // Figure out how much real data there is to read
            let section_data = self.section.raw_data();
            let real_data_remaining = section_data.len() - self.offset as usize;
            let real_read_size = if read_size > real_data_remaining {
                real_data_remaining
            } else {
                read_size
            };

            if real_read_size > 0 {
                let src_end = self.offset as usize + real_read_size;

                assert!(real_read_size <= buf.len() && src_end <= section_data.len());

                // Copy the requested data into the buffer
                buf[0..real_read_size].copy_from_slice(&section_data[self.offset as usize..src_end]);
            }

            // Fill any remaining data with zeros
            if real_read_size < read_size {
                for i in real_read_size..read_size {
                    debug_assert!(i <= buf.len());
                    buf[i] = 0;
                }
            }

            // Update the position and return the read size
            self.offset += read_size as u32;
            Ok(read_size)
        }
    }
}
