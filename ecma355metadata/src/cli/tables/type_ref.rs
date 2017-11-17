use std::io::Cursor;

use cli::{MetadataSizes, StringHandle, StringHandleReader};
use cli::tables::{TableHandle, TableIndex, TableMask, TableReader, TableHandleReader};
use error::Error;

pub struct TypeRef {
    pub resolution_scope: TableHandle,
    pub name: StringHandle,
    pub namespace: StringHandle,
}

pub struct TypeRefReader {
    resolution_scope_reader: TableHandleReader,
    string_reader: StringHandleReader,
}

impl TableReader for TypeRefReader {
    type Item = TypeRef;
    const INDEX: TableIndex = TableIndex::TypeRef;

    fn new(sizes: &MetadataSizes) -> TypeRefReader {
        TypeRefReader {
            resolution_scope_reader: index_reader!(sizes,
                0 => TableIndex::Module,
                1 => TableIndex::ModuleRef,
                2 => TableIndex::AssemblyRef,
                3 => TableIndex::TypeRef),
            string_reader: StringHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        self.resolution_scope_reader.size() + (2 * self.string_reader.size())
    }

    fn read(&self, buf: &[u8]) -> Result<TypeRef, Error> {
        let mut cursor = Cursor::new(buf);

        Ok(TypeRef {
            resolution_scope: self.resolution_scope_reader.read(&mut cursor)?,
            name: self.string_reader.read(&mut cursor)?,
            namespace: self.string_reader.read(&mut cursor)?,
        })
    }
}
