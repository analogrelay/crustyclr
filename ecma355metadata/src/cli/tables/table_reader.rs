use cli::MetadataSizes;
use cli::tables::TableIndex;
use error::Error;

pub trait TableReader {
    type Item;
    const INDEX: TableIndex;

    /// Creates a new TableReader using the provided sizes.
    fn new(sizes: &MetadataSizes) -> Self;

    /// Gets the size of a single row of data in this table.
    fn row_size(&self) -> usize;

    /// Reads an item from the provided buffer. The buffer is guaranteed to be exactly `self.row_size()` bytes in length.
    fn read(&self, buf: &[u8]) -> Result<Self::Item, Error>;
}