use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{MetadataSizes, ParamAttributes, StringHandle, StringHandleReader};
use cli::tables::{TableIndex, TableReader};

use error::Error;

pub struct Param {
    pub flags: ParamAttributes,
    pub sequence: u16,
    pub name: StringHandle,
}

pub struct ParamReader {
    string_reader: StringHandleReader,
}

impl TableReader for ParamReader {
    type Item = Param;
    const INDEX: TableIndex = TableIndex::Param;

    fn new(sizes: &MetadataSizes) -> ParamReader {
        ParamReader {
            string_reader: StringHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        (2 * size_of::<u16>()) + self.string_reader.size()
    }

    fn read(&self, mut buf: &[u8]) -> Result<Param, Error> {
        Ok(Param {
            flags: ParamAttributes::from_bits_truncate(buf.read_u16::<LittleEndian>()?),
            sequence: buf.read_u16::<LittleEndian>()?,
            name: self.string_reader.read(&mut buf)?,
        })
    }
}
