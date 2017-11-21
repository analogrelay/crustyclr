use std::io::Cursor;

use cli::{BlobHandle, BlobHeap, GuidHandle, GuidHeap, MetadataHeader, StreamHeader, StringHandle,
          StringHeap};
use cli::tables::TableStream;
use error::Error;
use Guid;

pub struct MetadataReader<'a> {
    metadata_header: MetadataHeader,
    string_heap: StringHeap<'a>,
    guid_heap: GuidHeap<'a>,
    blob_heap: BlobHeap<'a>,
    tables: TableStream<'a>,
}

impl<'a> MetadataReader<'a> {
    pub fn new(data: &'a [u8]) -> Result<MetadataReader<'a>, Error> {
        // Read the metadata and stream headers
        let mut cursor = Cursor::new(data);

        let metadata_header = MetadataHeader::read(&mut cursor)?;

        let mut table_stream = None;
        let mut string_heap = StringHeap::EMPTY;
        let mut guid_heap = GuidHeap::EMPTY;
        let mut blob_heap = BlobHeap::EMPTY;
        for _ in 0..metadata_header.streams {
            let header = StreamHeader::read(&mut cursor)?;
            let start = header.offset as usize;
            let end = start + (header.size as usize);
            match header.name.as_str() {
                "#~" => table_stream = Some(TableStream::new(&data[start..end])?),
                "#Strings" => string_heap = StringHeap::new(&data[start..end]),
                "#GUID" => guid_heap = GuidHeap::new(&data[start..end])?,
                "#Blob" => blob_heap = BlobHeap::new(&data[start..end]),
                _ => {}
            };
        }

        Ok(MetadataReader {
            metadata_header,
            string_heap,
            guid_heap,
            blob_heap,
            tables: table_stream.ok_or(Error::InvalidMetadata("No table stream was present"))?,
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

    pub fn get_blob(&self, handle: BlobHandle) -> Option<&[u8]> {
        self.blob_heap.get(handle.index())
    }
}
