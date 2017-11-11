use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidRef, HeapRef, MetadataSizes, StringRef};
use cli::tables::{TableIndex, TableRow, TableMask};
use error::Error;

pub struct TypeRef {
    pub resolution_scope: usize,
    pub name: StringRef,
    pub namespace: StringRef,
}

impl TableRow for TypeRef {
    const INDEX: TableIndex = TableIndex::TypeRef;

    fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<TypeRef, Error> {
        Ok(TypeRef {
            resolution_scope: sizes.read_coded_index(reader, TableMask::ResolutionScope)?,
            name: StringRef::read(reader, sizes)?,
            namespace: StringRef::read(reader, sizes)?,
        })
    }

    fn row_size(sizes: &MetadataSizes) -> usize {
        sizes.coded_index_size(TableMask::ResolutionScope) + StringRef::size(sizes) + StringRef::size(sizes)
    }
}

