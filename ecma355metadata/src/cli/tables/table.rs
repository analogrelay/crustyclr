use cli::tables::TableReader;
use error::Error;

pub struct Table<'a, T: TableReader> {
    data: &'a [u8],
    reader: Option<T>
}

impl<'a, T: TableReader> Table<'a, T> {
    pub const EMPTY: Table<'static, T> = Table::<'static, T> { data: &[], reader: None };

    pub fn new(data: &'a [u8], reader: T) -> Table<'a, T> {
        Table {
            data: data,
            reader: Some(reader),
        }
    }

    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {
            reader: self.reader.as_ref().unwrap(),
            buf: self.data
        }
    }

    pub fn len(&self) -> usize {
        self.data.len() / self.reader.as_ref().unwrap().row_size()
    }
}

pub struct Iter<'a, T: TableReader + 'a> {
    reader: &'a T,
    buf: &'a [u8]
}

impl<'a, T: TableReader + 'a> Iterator for Iter<'a, T> {
    type Item = Result<T::Item, Error>;

    fn next(&mut self) -> Option<Result<T::Item, Error>> {
        if self.buf.len() == 0 {
            None
        }
        else {
            // Slice off enough for a row
            let row_size = self.reader.row_size();
            let row = &self.buf[0..row_size];
            
            // Update the internal buffer
            self.buf = &self.buf[row_size..];

            // Read the item and return it
            Some(self.reader.read(row))
        }
    }
}