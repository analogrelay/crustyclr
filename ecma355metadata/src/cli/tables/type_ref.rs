use std::io::Read;

use cli::{MetadataSizes, StringHandle, StringHandleReader};
use cli::tables::{Table, TableDecoder, TableHandle, TableHandleReader, TableIndex, TableMask};
use error::Error;

pub struct TypeRef {
    pub resolution_scope: TableHandle,
    pub name: StringHandle,
    pub namespace: StringHandle,
}

impl Table for TypeRef {
    type Decoder = TypeRefDecoder;
    const INDEX: TableIndex = TableIndex::TypeRef;
}

pub struct TypeRefDecoder {
    count: usize,
    resolution_scope_reader: TableHandleReader,
    string_reader: StringHandleReader,
}

impl<R: Read> TableDecoder<R> for TypeRefDecoder {
    type Item = TypeRef;

    fn new(sizes: &MetadataSizes) -> TypeRefDecoder {
        TypeRefDecoder {
            count: sizes.row_count(Self::Item::INDEX),
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

    fn row_count(&self) -> usize {
        self.count
    }

    fn decode(&self, buf: &mut R) -> Result<TypeRef, Error> {
        Ok(TypeRef {
            resolution_scope: self.resolution_scope_reader.read(buf)?,
            name: self.string_reader.read(buf)?,
            namespace: self.string_reader.read(buf)?,
        })
    }
}
