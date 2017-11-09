use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataHeader, StreamHeader, TableIndex, TableMask};
use cli::tables;
use error::Error;

pub struct MetadataReader<'a> {
    metadata_header: MetadataHeader,
    heap_sizes: HeapSizes,
    module_table: tables::ModuleTable<'a>,
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

        // Load row counts
        let mut row_counts = Vec::new();
        for idx in TableIndex::each() {
            if valid_mask.has_table(idx) {
                row_counts.push(cursor.read_u32::<LittleEndian>()?);
            }
        }

        // Get the position of the cursor and re-slice the data to get the rows
        let mut rows = &data[cursor.position() as usize..];

        let mut row_iter = row_counts.iter();

        Ok(MetadataReader {
            metadata_header: metadata_header,
            heap_sizes: heap_sizes,
            module_table: get_module_table(&mut rows, heap_sizes, valid_mask, &mut row_iter)?,
        })
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn heap_sizes(&self) -> HeapSizes {
        self.heap_sizes
    }

    pub fn module_table(&self) -> &tables::ModuleTable<'a> {
        &self.module_table
    }
}

fn get_module_table<'a>(
    rows: &mut &'a [u8],
    heap_sizes: HeapSizes,
    valid_mask: TableMask,
    row_iter: &mut ::std::slice::Iter<u32>,
) -> Result<tables::ModuleTable<'a>, Error> {
    if valid_mask.has_table(TableIndex::Module) {
        let row_count = row_iter.next().ok_or(Error::InvalidMetadata)?;

        // Determine the total size
        let total_size = tables::ModuleTable::row_size(heap_sizes) * (*row_count) as usize;
        let module_rows = &rows[0..total_size];

        // Advance rows
        *rows = &rows[total_size..];

        Ok(tables::ModuleTable::new(module_rows, heap_sizes))
    } else {
        Ok(tables::ModuleTable::empty())
    }
}
