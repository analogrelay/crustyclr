use std::io::Read;

use cli::HeapSizes;
use cli::tables::TableIndex;
use error::Error;

pub trait TableRow {
    const INDEX: TableIndex;

    fn size(heap_sizes: HeapSizes) -> usize;
    fn read<R: Read>(reader: &mut R, heap_sizes: HeapSizes) -> Result<Self, Error>;
}