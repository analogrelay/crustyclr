use std::ops::Index;

use cli::{MetadataHeader, MetadataSizes};
use cli::tables::{TableIndex, TableReader};
use error::Error;

const EMPTY_DATA: [u8; 0] = [];

pub struct TableStream<'a> {
    metadata_sizes: MetadataSizes,
    module: ModuleReader,
}

impl<'a> TableStream<'a> {
    const EMPTY: TableStream<'static> = TableStream { data: None };

    pub fn new(data: &'a [u8]) -> Result<TableStream<'a>, Error> {
        let mut cursor = Cursor::new(data);

        let sizes = MetadataSizes::read(&mut cursor)?;

        Ok(TableStream {
            metadata_sizes: sizes,
            data: rows,
        })
    }

    pub fn metadata_sizes(&self) -> &MetadataSizes {
        &self.metadata_sizes
    }
}