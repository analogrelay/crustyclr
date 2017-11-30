use std::ops::Deref;
use std::io::Read;

use pe::{DirectoryType, PeImage};
use cli::{CliHeader, MetadataHeader};
use error::Error;

pub struct MetadataImage<D: Deref<Target = [u8]>> {
    pe: PeImage<D>,
}

impl<D: Deref<Target = [u8]>> MetadataImage<D> {
    pub fn load_data(data: D) -> Result<MetadataImage<D>, Error> {
        MetadataImage::load(PeImage::load(data)?)
    }

    pub fn load(pe: PeImage<D>) -> Result<MetadataImage<D>, Error> {
        // // Load the CLI header
        // let cli_header = {
        //     let cli_header_dir = pe.get_directory(DirectoryType::CliHeader)
        //         .ok_or(Error::CliHeaderNotFound)?;

        //     // Map the virtual range to a physical one
        //     let phys = pe.map_virtual_range(cli_header_dir.range)
        //         .ok_or(Error::CliHeaderNotFound)?;

        //     // Seek and load the Cli Header
        //     reader.seek(SeekFrom::Start(phys.start as u64))?;
        //     CliHeader::read(&mut reader)?
        // };

        // // Load the metadata header
        // let (metadata_start, metadata_header) = {
        //     let phys = pe.map_virtual_range(cli_header.metadata)
        //         .ok_or(Error::CliHeaderNotFound)?;

        //     // Seek and load the Metadata Header
        //     reader.seek(SeekFrom::Start(phys.start as u64))?;
        //     (phys.start, MetadataHeader::read(&mut reader)?)
        // };

        // // Load metadata sizes from the "#~" stream
        // let (tables_start, metadata_sizes) = {
        //     let stream = metadata_header
        //         .get_stream("#~")
        //         .ok_or(Error::InvalidMetadata(
        //             "Image does not contain a '#~' metadata stream",
        //         ))?;

        //     // Seek to the stream
        //     let start = metadata_start + stream.offset;
        //     reader.seek(SeekFrom::Start(start as u64))?;
        //     let sizes = MetadataSizes::read(&mut reader)?;

        //     // Load the metadata sizes and record the offset of the start of tables
        //     (reader.seek(SeekFrom::Current(0))?, sizes)
        // };

        Ok(MetadataImage {
            pe,
        })
    }

    pub fn pe(&self) -> &PeImage<D> {
        &self.pe
    }

    // pub fn cli_header(&self) -> &CliHeader {
    //     &self.cli_header
    // }

    // pub fn metadata_header(&self) -> &MetadataHeader {
    //     &self.metadata_header
    // }
}

impl MetadataImage<Vec<u8>> {
    pub fn read<R: Read>(reader: R) -> Result<MetadataImage<Vec<u8>>, Error> {
        MetadataImage::load(PeImage::read(reader)?)
    }
}
