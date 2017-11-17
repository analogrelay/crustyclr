use std::io::Cursor;
use std::mem;

use cli::{GuidHandle, GuidHeap, HeapHandle, MetadataHeader, MetadataSizes, StreamHeader,
          StringHandle, StringHeap};
use cli::tables::{self, TableStream}
use error::Error;
use Guid;

pub struct MetadataReader<'a> {
    metadata_header: MetadataHeader,
    string_heap: StringHeap<'a>,
    guid_heap: GuidHeap<'a>,
    tables: TableStream<'a>,
}

impl<'a> MetadataReader<'a> {
    pub fn new(data: &'a [u8]) -> Result<MetadataReader<'a>, Error> {
        // Read the metadata and stream headers
        let mut cursor = Cursor::new(data);

        let metadata_header = MetadataHeader::read(&mut cursor)?;

        let mut table_stream = TableStream::EMPTY;
        let mut string_heap = StringHeap::EMPTY;
        let mut guid_heap = GuidHeap::EMPTY;
        for _ in 0..metadata_header.streams {
            let header = StreamHeader::read(&mut cursor)?;
            let start = header.offset as usize;
            let end = start + (header.size as usize);
            match header.name.as_str() {
                "#~" => table_stream = TableStream::new(&data[start..end])?,
                "#Strings" => string_heap = StringHeap::new(&data[start..end]),
                "#GUID" => guid_heap = GuidHeap::new(&data[start..end])?,
                _ => {}
            };
        }

        Ok(MetadataReader {
            metadata_header: metadata_header,
            string_heap: string_heap,
            guid_heap: guid_heap,
            tables: table_stream,
        })
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn tables(&self) -> &TableStream {
        &self.tables
    }

    pub fn get_string(&self, handle: StringHandle) -> Option<&[u8]> {
        self.string_heap.get(handle.index())
    }

    pub fn get_guid(&self, handle: GuidHandle) -> Option<&Guid> {
        self.guid_heap.get(handle.index())
    }
}