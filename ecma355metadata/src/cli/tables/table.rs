use std::io::{self, Cursor, Read};
use std::marker::PhantomData;

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

    /// Reads an item from the provided buffer. The buffer is guaranteed to be `self.row_size()` bytes in length.
    fn read(&self, buf: &[u8]) -> Result<Item, Error>;
}

// pub trait TableRow: Sized {
//     const INDEX: TableIndex;

//     fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<Self, Error>;
//     fn row_size(sizes: &MetadataSizes) -> usize;
// }

// pub struct Table<'a, T: TableRow> {
//     rows: Option<&'a [u8]>,
//     sizes: Option<&'a MetadataSizes>,
//     row_size: usize,
//     _phantom: PhantomData<T>,
// }

// impl<'a, T: TableRow> Table<'a, T> {
//     pub fn new(rows: &'a [u8], sizes: &'a MetadataSizes) -> Table<'a, T> {
//         Table {
//             rows: Some(rows),
//             sizes: Some(sizes),
//             row_size: T::row_size(sizes),
//             _phantom: PhantomData,
//         }
//     }

//     pub fn empty() -> Table<'a, T> {
//         Table {
//             rows: None,
//             sizes: None,
//             row_size: 0,
//             _phantom: PhantomData,
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.rows.is_none()
//     }

//     pub fn len(&self) -> usize {
//         match self.rows {
//             Some(ref r) => r.len() / self.row_size,
//             None => 0,
//         }
//     }

//     pub fn iter(&'a self) -> Iter<'a, T> {
//         match self.rows {
//             None => Iter {
//                 cursor: None,
//                 table: self,
//             },
//             Some(r) => Iter {
//                 cursor: Some(Cursor::new(r)),
//                 table: self,
//             },
//         }
//     }
// }

// pub struct Iter<'a, T: TableRow + 'a> {
//     cursor: Option<Cursor<&'a [u8]>>,
//     table: &'a Table<'a, T>,
// }

// impl<'a, T: TableRow + 'a> Iterator for Iter<'a, T> {
//     type Item = Result<T, Error>;

//     fn next(&mut self) -> Option<Result<T, Error>> {
//         if let Some(ref mut cur) = self.cursor {
//             let res = T::read(
//                 cur,
//                 self.table
//                     .sizes
//                     .expect("Metadata Sizes cannot be None when Data is Some!"),
//             );
//             match res {
//                 Err(Error::IoError(ref io)) if io.kind() == io::ErrorKind::UnexpectedEof => None,
//                 x => Some(x),
//             }
//         } else {
//             None
//         }
//     }
// }
