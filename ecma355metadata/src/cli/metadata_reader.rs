use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataHeader, StreamHeader, TableMask};
use cli::tables;
use error::Error;

pub struct MetadataReader<'a> {
    metadata_header: MetadataHeader,
    heap_sizes: HeapSizes,
    module_cursor: Cursor<&'a [u8]>,
}

impl<'a> MetadataReader<'a> {
    pub fn new(data: &'a [u8]) -> Result<MetadataReader<'a>, Error> {
        // Read the metadata and stream headers
        let mut cursor = Cursor::new(data);

        let metadata_header = MetadataHeader::read(&mut cursor)?;

        let mut stream_headers = Vec::with_capacity(metadata_header.streams as usize);
        for _ in 0..metadata_header.streams {
            stream_headers.push(StreamHeader::read(&mut cursor)?);
        }

        let metadata_stream = stream_headers
            .iter()
            .find(|h| h.name == "#~")
            .ok_or(Error::StreamNotFound)?;
        let start = metadata_stream.offset as usize;
        let end = start + (metadata_stream.size as usize);
        let metadata = &data[start..end];

        let mut cursor = Cursor::new(metadata);

        // Skip reserved value, and version numbers
        cursor.read_u32::<LittleEndian>()?;
        cursor.read_u8()?;
        cursor.read_u8()?;
        let heap_sizes = HeapSizes::from_bits_truncate(cursor.read_u8()?);

        // Skip reserved value
        cursor.read_u8()?;

        // Read valid and sorted vectors
        let valid_mask = TableMask::from_bits_truncate(cursor.read_u64::<LittleEndian>()?);
        let sorted_mask = TableMask::from_bits_truncate(cursor.read_u64::<LittleEndian>()?);

        Ok(MetadataReader {
            metadata_header: metadata_header,
            heap_sizes: heap_sizes,
            module_cursor: cursor,
        })
    }

    pub fn modules(&self) -> Box<Iterator<Item = Result<tables::Module, Error>>> {
        let module_reader = tables::ModuleTable::new(self.module_cursor.clone(), self.heap_sizes);
        Box::new(module_reader)
    }
}
