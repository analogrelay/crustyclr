use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidHandle, HeapHandle, MetadataSizes, StringHandle};
use cli::tables::{TableIndex, TableRow};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringHandle,
    pub mvid: GuidHandle,
    pub enc_id: GuidHandle,
    pub enc_base_id: GuidHandle,
}

impl TableRow for Module {
    const INDEX: TableIndex = TableIndex::Module;

    fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<Module, Error> {
        Ok(Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: StringHandle::read(reader, sizes)?,
            mvid: GuidHandle::read(reader, sizes)?,
            enc_id: GuidHandle::read(reader, sizes)?,
            enc_base_id: GuidHandle::read(reader, sizes)?,
        })
    }

    fn row_size(sizes: &MetadataSizes) -> usize {
        size_of::<u16>() + StringHandle::size(sizes) + GuidHandle::size(sizes)
            + GuidHandle::size(sizes) + GuidHandle::size(sizes)
    }
}
