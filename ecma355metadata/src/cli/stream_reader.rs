use std::io::{Error, Read, Seek, SeekFrom};

use pe::SectionReader;

pub struct StreamReader<'a> {
    section_reader: SectionReader<'a>,
    start: u32,
    size: u32,
}

impl<'a> StreamReader<'a> {
    pub fn new(section_reader: SectionReader<'a>, start: u32, size: usize) -> StreamReader {
        StreamReader {
            section_reader: section_reader,
            start: start,
            size: size,
        }
    }
}

impl<'a> Seek for StreamReader<'a> {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        match pos {
            SeekFrom::Start(x) => self.section_reader.seek(SeekFrom::Start(self.start + x)),
            SeekFrom::End(x) => self.section_reader
                .seek(SeekFrom::Start((self.start + self.size) - x)),
            SeekFrom::Current(x) => self.section_reader.seek(SeekFrom::Current(x)),
        }
    }
}

impl<'a> Read for StreamReader<'a> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        // Clamp the read size to whatever is left
        let remaining_to_read = self.size - (self.section_reader.offset - self.start);
        let read_size = if buf.len() > remaining_to_read {
            remaining_to_read
        } else {
            buf.len()
        };

        // If there isn't any data to be read, return EOF
        if read_size == 0 {
            Ok(0)
        } else {
            // Do the read
            self.section_reader.read(buf)
        }
    }
}
