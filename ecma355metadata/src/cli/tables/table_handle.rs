use std::io::Read;
use std::fmt;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{MetadataSizes, SMALL_INDEX_SIZE};
use cli::tables::{TableIndex, TableMask};
use error::Error;

pub struct TableHandle {
    index: usize,
    table: TableIndex,
}

impl TableHandle {
    pub fn new(index: usize, table: TableIndex) -> TableHandle {
        TableHandle {
            index: index,
            table: table,
        }
    }

    pub fn read<R: Read>(
        reader: &mut R,
        table: TableIndex,
        sizes: &MetadataSizes,
    ) -> Result<TableHandle, Error> {
        if sizes.index_size(table) == SMALL_INDEX_SIZE {
            Ok(TableHandle::new(
                reader.read_u16::<LittleEndian>()? as usize,
                table,
            ))
        } else {
            Ok(TableHandle::new(
                reader.read_u32::<LittleEndian>()? as usize,
                table,
            ))
        }
    }
}

impl fmt::Display for TableHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}[0x{:04X}]", self.table, self.index)
    }
}
