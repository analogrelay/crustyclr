use std::io::Cursor;
use std::mem;

use cli::{GuidHeap, MetadataHeader, MetadataSizes, StreamHeader, StringHeap};
use cli::tables::{self, Table, TableIndex, TableRow};
use error::Error;

pub struct MetadataReader<'a> {
    metadata_header: MetadataHeader,
    metadata_sizes: MetadataSizes,
    string_heap: StringHeap<'a>,
    guid_heap: GuidHeap<'a>,
    table_data: Vec<Option<&'a [u8]>>,
}

impl<'a> MetadataReader<'a> {
    pub fn new(data: &'a [u8]) -> Result<MetadataReader<'a>, Error> {
        // Read the metadata and stream headers
        let mut cursor = Cursor::new(data);

        let metadata_header = MetadataHeader::read(&mut cursor)?;

        let mut metadata_stream: &[u8] = &[0u8; 0];
        let mut string_heap = StringHeap::empty();
        let mut guid_heap = GuidHeap::empty();
        for _ in 0..metadata_header.streams {
            let header = StreamHeader::read(&mut cursor)?;
            let start = header.offset as usize;
            let end = start + (header.size as usize);
            match header.name.as_str() {
                "#~" => {
                    metadata_stream = &data[start..end];
                }
                "#Strings" => string_heap = StringHeap::new(&data[start..end]),
                "#GUID" => {
                    let guid_data = &data[start..end];
                    // Make sure the data is a multiple of 16 in length
                    if guid_data.len() % 16 != 0 {
                        return Err(Error::InvalidMetadata(
                            "GUID stream is not a multiple of 16 bytes in length.",
                        ));
                    }
                    guid_heap = GuidHeap::new(unsafe { mem::transmute(guid_data) });
                }
                _ => {}
            };
        }

        if metadata_stream.len() == 0 {
            return Err(Error::StreamNotFound);
        }

        let mut cursor = Cursor::new(metadata_stream);

        let sizes = MetadataSizes::read(&mut cursor)?;

        // Get the position of the cursor and re-slice the data to get the rows
        let rows = &metadata_stream[cursor.position() as usize..];

        let table_data = load_tables(rows, &sizes);

        Ok(MetadataReader {
            metadata_header: metadata_header,
            metadata_sizes: sizes,
            string_heap: string_heap,
            guid_heap: guid_heap,
            table_data: table_data,
        })
    }

    pub fn metadata_header(&self) -> &MetadataHeader {
        &self.metadata_header
    }

    pub fn metadata_sizes(&self) -> &MetadataSizes {
        &self.metadata_sizes
    }

    pub fn table<'b, T: TableRow>(&'b self) -> Table<'b, T> {
        let idx = T::INDEX as usize;
        if idx > self.table_data.len() {
            Table::empty()
        } else if let Some(ref data) = self.table_data[idx] {
            Table::new(data, &self.metadata_sizes)
        } else {
            Table::empty()
        }
    }

    pub fn string_heap(&self) -> &StringHeap<'a> {
        &self.string_heap
    }

    pub fn guid_heap(&self) -> &GuidHeap<'a> {
        &self.guid_heap
    }
}

fn load_tables<'a>(mut rows: &'a [u8], sizes: &MetadataSizes) -> Vec<Option<&'a [u8]>> {
    let mut tables = Vec::new();
    tables.push(get_table_data::<tables::Module>(&mut rows, sizes));

    tables
}

fn get_table_data<'a, T: TableRow>(rows: &mut &'a [u8], sizes: &MetadataSizes) -> Option<&'a [u8]> {
    let row_count = sizes.row_count(T::INDEX);
    if row_count > 0 {
        // Determine the total size
        let total_size = T::row_size(sizes) * row_count;
        let module_rows = &rows[0..total_size];

        // Advance rows
        *rows = &rows[total_size..];

        Some(module_rows)
    } else {
        None
    }
}
