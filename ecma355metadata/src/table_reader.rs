use std::io::Read;
use std::marker::PhantomData;

use cli::MetadataSizes;
use cli::tables::Table;

pub struct TableReader<'a, T: Table, R: Read> {
    reader: &'a mut R,
    offset: usize,
    decoder: T::Decoder,
    _phantom: PhantomData<T>,
}

impl<'a, T: Table, R: Read> TableReader<'a, T, R> {
    pub fn new(reader: &'a mut R, metadata_sizes: &MetadataSizes) -> TableReader<'a, T, R> {
        let decoder = T::Decoder::new(metadata_sizes);
        TableReader {
            reader,
            offset: 0,
            decoder,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T: TableItem, R: Read> Iterator for TableReader<'a, T, R> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.offset >= self.decoder.row_count() {
            None
        } else {
            let buf = 
        }
    }
}
