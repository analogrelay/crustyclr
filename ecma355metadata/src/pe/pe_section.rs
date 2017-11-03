use pe::{SectionHeader, SectionReader};

pub struct PeSection {
    header: SectionHeader,
    data: Vec<u8>,
}

impl PeSection {
    pub fn new(header: SectionHeader, data: Vec<u8>) -> PeSection {
        PeSection {
            header: header,
            data: data,
        }
    }

    pub fn header(&self) -> &SectionHeader {
        &self.header
    }

    pub fn raw_data(&self) -> &[u8] {
        self.data.as_slice()
    }

    pub fn contains_rva(&self, rva: u32) -> bool {
        rva >= self.header.virtual_address && rva <= self.header.virtual_end()
    }

    pub fn create_reader<'a>(&'a self) -> SectionReader<'a> {
        SectionReader {
            section: self,
            offset: 0,
        }
    }
}
