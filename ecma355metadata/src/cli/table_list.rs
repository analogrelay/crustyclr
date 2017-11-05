use std::io::Read;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use error::Error;

use cli::{HeapSizes, TableIndex, TableMask};

pub struct TableHeader {
    pub table: TableIndex,
    pub sorted: bool,
    pub rows: u32,
}

pub struct TableList {
    pub major_version: u8,
    pub minor_version: u8,
    pub heap_sizes: HeapSizes,
    tables: Vec<TableHeader>,
}

impl TableList {
    pub fn read<A: Read>(buf: &mut A) -> Result<TableList, Error> {
        // Skip reserved value
        buf.read_u32::<LittleEndian>()?;

        let major_version = buf.read_u8()?;
        let minor_version = buf.read_u8()?;
        let heap_sizes = HeapSizes::from_bits_truncate(buf.read_u8()?);

        // Skip reserved value
        buf.read_u8()?;

        // Read valid and sorted vectors
        let valid_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);
        let sorted_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);

        // Determine which tables are present, and which are sorted
        let mut tables = Vec::new();
        for idx in TableIndex::each() {
            if valid_mask.has_table(idx) {
                tables.push(TableHeader {
                    table: idx,
                    sorted: sorted_mask.has_table(idx),
                    rows: buf.read_u32::<LittleEndian>()?,
                })
            }
        }

        Ok(TableList {
            major_version: major_version,
            minor_version: minor_version,
            heap_sizes: heap_sizes,
            tables: tables
        })
    }

    pub fn tables(&self) -> &Vec<TableHeader> {
        &self.tables
    }
}
