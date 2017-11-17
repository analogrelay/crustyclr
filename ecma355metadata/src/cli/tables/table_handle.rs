use std::io::Read;
use std::fmt;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{SMALL_INDEX_SIZE, LARGE_INDEX_SIZE};
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
}

impl fmt::Display for TableHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}[0x{:04X}]", self.table, self.index)
    }
}

pub struct TableHandleReader  {
    is_large: bool,
    tag_mask: usize,
    shift_distance: usize,
    table_map: fn(usize) -> Option<TableIndex>,
}

impl TableHandleReader {
    pub fn new(is_large: bool, tables: TableMask, table_map: fn(usize) -> Option<TableIndex>) -> TableHandleReader {
        assert!(tables.bits() != 0);

        // Calculate the tag mask and shift distance
        let table_count = tables.bits().count_ones();
        let mut current = table_count - 1;
        let mut tag_mask = 0;
        let mut shift_distance = 0;
        while current > 0 {
            tag_mask = (tag_mask << 1) | 1;
            current >>= 1;
            shift_distance += 1;
        }

        TableHandleReader {
            is_large,
            tag_mask,
            shift_distance,
            table_map,
        }
    }

    pub fn size(&self) -> usize {
        if self.is_large {
            LARGE_INDEX_SIZE
        } else {
            SMALL_INDEX_SIZE
        }
    }

    pub fn read<R: Read>(&self, reader: &mut R) -> Result<TableHandle, Error> {
        let val =
            if self.is_large {
                reader.read_u32::<LittleEndian>()? as usize
            } else {
                reader.read_u16::<LittleEndian>()? as usize
            };

        let tag = val & self.tag_mask;
        let index = val >> self.shift_distance;

        let table = (self.table_map)(tag).ok_or(Error::InvalidCodedIndex)?;
        Ok(TableHandle::new(index, table))
    }
}
