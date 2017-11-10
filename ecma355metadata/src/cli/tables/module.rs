use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidRef, HeapRef, MetadataSizes, StringRef};
use cli::tables::{TableIndex, TableRow};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringRef,
    pub mvid: GuidRef,
    pub enc_id: GuidRef,
    pub enc_base_id: GuidRef,
}

impl TableRow for Module {
    const INDEX: TableIndex = TableIndex::Module;

    fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<Module, Error> {
        Ok(Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: StringRef::read(reader, sizes)?,
            mvid: GuidRef::read(reader, sizes)?,
            enc_id: GuidRef::read(reader, sizes)?,
            enc_base_id: GuidRef::read(reader, sizes)?,
        })
    }

    fn row_size(sizes: &MetadataSizes) -> usize {
        size_of::<u16>() + StringRef::size(sizes) + GuidRef::size(sizes) + GuidRef::size(sizes)
            + GuidRef::size(sizes)
    }
}
