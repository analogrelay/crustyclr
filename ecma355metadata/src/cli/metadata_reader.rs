use std::io::{Seek, SeekFrom};

use error::Error;

use pe::{DirectoryType, PeImage, SectionReader};
use cli::{CliHeader, MetadataHeader, MetadataStreamHeader, StreamReader, TableList};

pub struct MetadataReader {
    pe_image: PeImage,
    cli_header: CliHeader,
    metadata_header: MetadataHeader,
    stream_headers: Vec<MetadataStreamHeader>,
    table_list: Option<TableList>
}

impl MetadataReader {
    pub fn new(mut pe: PeImage) -> Result<MetadataReader, Error> {
        // Locate and load the CLI header
        let cli_header = load_cli_header(&mut pe)?;

        let (metadata_header, stream_headers) = {
            // Locate and load metadata_header
            let mut reader = pe.create_reader(cli_header.metadata.rva)?;
            let metadata_header = MetadataHeader::read(&mut reader)?;

            // Load stream headers
            let mut stream_headers = Vec::with_capacity(metadata_header.streams as usize);
            for _ in 0..metadata_header.streams {
                stream_headers.push(MetadataStreamHeader::read(&mut reader)?);
            }

            (metadata_header, stream_headers)
        };
        
        let mut reader = MetadataReader {
            pe_image: pe,
            cli_header: cli_header,
            metadata_header: metadata_header,
            stream_headers: stream_headers,
            table_list: None
        };

        reader.load_tables()?;

        Ok(reader)
    }

    pub fn cli_header(&self) -> &CliHeader {
        &self.cli_header
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn stream_headers(&self) -> &Vec<MetadataStreamHeader> {
        &self.stream_headers
    }

    pub fn table_list(&self) -> &TableList {
        // This is guaranteed to be Some(), because new() initializes it.
        self.table_list.as_ref().expect("MetadataReader was not properly initialized. The table list wasn't loaded!")
    }

    pub fn get_stream<'a>(&'a mut self, name: &str) -> Result<StreamReader<'a>, Error> {
        let (offset, size) = {
            if let Some(header) = self.stream_headers.iter().find(|s| s.name == name) {
                (self.cli_header.metadata.rva + header.offset, header.size)
            } else {
                return Err(Error::StreamNotFound)
            }
        };

        let mut section_reader = self.pe_image.create_reader(offset)?;

        // Wrap it in a stream reader
        Ok(StreamReader::new(section_reader, size))
    }

    fn load_tables(&mut self) -> Result<(), Error> {
        let table_header = {
            // Get a reader for "#~"
            let mut table_stream_reader = self.get_stream("#~")?;

            // Read the table header
            TableList::read(&mut table_stream_reader)?
        };

        self.table_list = Some(table_header);

        Ok(())
    }
}

fn load_cli_header(pe: &mut PeImage) -> Result<CliHeader, Error> {
    let cli_header_rva = pe.get_directory(DirectoryType::CliHeader)
        .map(|d| d.range.rva)
        .ok_or(Error::CliHeaderNotFound)?;

    let mut reader = pe.create_reader(cli_header_rva)?;

    Ok(CliHeader::read(&mut reader)?)
}
