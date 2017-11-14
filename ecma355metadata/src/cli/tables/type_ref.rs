use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapHandle, MetadataSizes, StringHandle, LARGE_INDEX_SIZE};
use cli::tables::{TableHandle, TableIndex, TableMask, TableRow};
use error::Error;

pub struct TypeRef {
    pub resolution_scope: TableHandle,
    pub name: StringHandle,
    pub namespace: StringHandle,
}

impl TableRow for TypeRef {
    const INDEX: TableIndex = TableIndex::TypeRef;

    fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<TypeRef, Error> {
        let resolution_scope =
            if sizes.coded_index_size(TableMask::ResolutionScope) == LARGE_INDEX_SIZE {
                reader.read_u32::<LittleEndian>()? as usize
            } else {
                reader.read_u16::<LittleEndian>()? as usize
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
            name: StringHandle::read(reader, sizes)?,
            namespace: StringHandle::read(reader, sizes)?,
        })
    }

    fn row_size(sizes: &MetadataSizes) -> usize {
        sizes.coded_index_size(TableMask::ResolutionScope) + StringHandle::size(sizes)
            + StringHandle::size(sizes)
    }
}
