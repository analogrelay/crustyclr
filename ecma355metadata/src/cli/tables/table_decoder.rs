use std::io::Read;

use cli::MetadataSizes;
use cli::tables::TableIndex;
use error::Error;

pub trait TableDecoder<R: Read> {
    type Item;

    /// Creates a new TableReader using the provided sizes.
    fn new(sizes: &MetadataSizes) -> Self;

    /// Gets the size of a single row of data in this table.
    fn row_size(&self) -> usize;

    /// Gets the number of rows in this table.
    fn row_count(&self) -> usize;

    /// Reads an item from the provided buffer. The buffer is guaranteed to be exactly `self.row_size()` bytes in length.
    fn decode(&self, buf: &mut R) -> Result<Self::Item, Error>;
}