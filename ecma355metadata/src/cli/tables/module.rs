use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapRef, StringRef, GuidRef};
use cli::tables::{TableRow, TableIndex};
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

    fn size(heap_sizes: HeapSizes) -> usize {
        size_of::<u16>() +
            StringRef::size(heap_sizes) +
            GuidRef::size(heap_sizes) +
            GuidRef::size(heap_sizes) +
            GuidRef::size(heap_sizes)
    }

    fn read<R: Read>(reader: &mut R, heap_sizes: HeapSizes) -> Result<Self, Error> {
        Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: StringRef::read(reader),
            mvid: GuidRef::read(reader),
            enc_id: GuidRef::read(reader),
            enc_base_id: GuidRef::read(reader),
        }
    }
}