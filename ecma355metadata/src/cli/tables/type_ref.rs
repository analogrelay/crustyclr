use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{MetadataSizes, StringHandle, StringHandleReader, LARGE_INDEX_SIZE};
use cli::tables::{TableHandle, TableIndex, TableMask, TableReader};
use error::Error;

pub struct TypeRef {
    pub resolution_scope: TableHandle,
    pub name: StringHandle,
    pub namespace: StringHandle,
}

pub struct TypeRefReader {
    resolution_scope_size: usize,
    string_reader: StringHandleReader,
}

impl TableReader for TypeRefReader {
    type Item = TypeRef;
    const INDEX: TableIndex = TableIndex::TypeRef;

    fn new(sizes: &MetadataSizes) -> TypeRefReader {
        TypeRefReader {
            resolution_scope_size: sizes.coded_index_size(TableMask::ResolutionScope),
            string_reader: StringHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        self.resolution_scope_size + (2 * self.string_reader.size())
    }

    fn read(&self, buf: &[u8]) -> Result<TypeRef, Error> {
        let mut cursor = Cursor::new(buf);

        let resolution_scope =
            if self.resolution_scope_size == LARGE_INDEX_SIZE {
                cursor.read_u32::<LittleEndian>()? as usize
            } else {
                cursor.read_u16::<LittleEndian>()? as usize
            };

        // Mask off the bottom two bits
        let tag = resolution_scope & 0b11;
        let index = resolution_scope >> 2;

        let table = match tag {
            0 => TableIndex::Module,
            1 => TableIndex::ModuleRef,
            2 => TableIndex::AssemblyRef,
            3 => TableIndex::TypeRef,
            _ => return Err(Error::InvalidCodedIndex),
        };

        Ok(TypeRef {
            resolution_scope: TableHandle::new(index, table),
            name: self.string_reader.read(&mut cursor)?,
            namespace: self.string_reader.read(&mut cursor)?,
        })
    }
}
