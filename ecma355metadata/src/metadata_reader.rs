use std::io::{Read, Seek};

use format::{CliHeader, DirectoryType, MetadataHeader};

use error::Error;

use PeReader;

pub struct MetadataReader<R: Read + Seek> {
    pe_reader: PeReader<R>,
    cli_header: CliHeader,
    metadata_header: MetadataHeader
}

impl<R: Read + Seek> MetadataReader<R> {
    pub fn new(stream: R) -> Result<MetadataReader<R>, Error> {
        // Read the PE file
        let mut pe_reader = PeReader::new(stream)?;

        // Locate and load the CLI header
        let cli_header = load_cli_header(&mut pe_reader)?; 

        // Locate the metadata root and read the metadata header
        pe_reader.seek_rva(cli_header.metadata.rva)?;
        let metadata_header = MetadataHeader::read(&mut pe_reader)?;

        Ok(MetadataReader {
            pe_reader: pe_reader,
            cli_header: cli_header,
            metadata_header: metadata_header,
        })
    }

    pub fn cli_header(&self) -> &CliHeader {
        &self.cli_header
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }
}

fn load_cli_header<R: Read + Seek>(pe_reader: &mut PeReader<R>) -> Result<CliHeader, Error> {
    // Locate the CLI header directory
    let cli_header_rva = pe_reader
        .pe_header()
        .ok_or(Error::NotAPortableExecutable)?
        .directories()
        .iter()
        .find(|d| d.directory_type == DirectoryType::CliHeader)
        .ok_or(Error::CliHeaderNotFound)?
        .rva;

    // Seek to it and read it
    pe_reader.seek_rva(cli_header_rva)?;
    Ok(CliHeader::read(pe_reader)?)
}