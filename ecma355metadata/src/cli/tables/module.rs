use std::io::{self, Cursor, Read};
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidRef, HeapRef, HeapSizes, StringRef};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringRef,
    pub mvid: GuidRef,
    pub enc_id: GuidRef,
    pub enc_base_id: GuidRef,
}

impl Module {
    pub fn read<R: Read>(reader: &mut R, heap_sizes: HeapSizes) -> Result<Module, Error> {
        Ok(Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: StringRef::read(reader, heap_sizes)?,
            mvid: GuidRef::read(reader, heap_sizes)?,
            enc_id: GuidRef::read(reader, heap_sizes)?,
            enc_base_id: GuidRef::read(reader, heap_sizes)?,
        })
    }
}

pub struct ModuleTable<'a> {
    rows: Option<&'a [u8]>,
    heap_sizes: HeapSizes,
    row_size: usize,
}

impl<'a> ModuleTable<'a> {
    pub fn new(rows: &'a [u8], heap_sizes: HeapSizes) -> ModuleTable<'a> {
        ModuleTable {
            rows: Some(rows),
            heap_sizes: heap_sizes,
            row_size: ModuleTable::row_size(heap_sizes),
        }
    }

    pub fn empty() -> ModuleTable<'a> {
        ModuleTable {
            rows: None,
            heap_sizes: HeapSizes::empty(),
            row_size: 0,
        }
    }

    pub fn row_size(heap_sizes: HeapSizes) -> usize {
        size_of::<u16>() + StringRef::size(heap_sizes) + GuidRef::size(heap_sizes)
            + GuidRef::size(heap_sizes) + GuidRef::size(heap_sizes)
    }

    pub fn present(&self) -> bool {
        self.rows.is_some()
    }

    pub fn len(&self) -> usize {
        match self.rows {
            Some(ref r) => r.len() / self.row_size,
            None => 0,
        }
    }

    pub fn iter(&'a self) -> Iter<'a> {
        match self.rows {
            None => Iter {
                cursor: None,
                table: self,
            },
            Some(r) => Iter {
                cursor: Some(Cursor::new(r)),
                table: self,
            },
        }
    }
}

pub struct Iter<'a> {
    cursor: Option<Cursor<&'a [u8]>>,
    table: &'a ModuleTable<'a>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Result<Module, Error>;

    fn next(&mut self) -> Option<Result<Module, Error>> {
        if let Some(ref mut cur) = self.cursor {
            match Module::read(cur, self.table.heap_sizes) {
                Err(Error::IoError(ref io)) if io.kind() == io::ErrorKind::UnexpectedEof => None,
                x => Some(x),
            }
        } else {
            None
        }
    }
}
