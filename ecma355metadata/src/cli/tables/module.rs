use std::io::{Read, Cursor};
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidHandle, HeapHandle, MetadataSizes, StringHandle, StringHandleReader, GuidHandleReader};
use cli::tables::{TableIndex, TableRow};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringHandle,
    pub mvid: GuidHandle,
    pub enc_id: GuidHandle,
    pub enc_base_id: GuidHandle,
}

pub struct ModuleReader {
    string_reader: StringHandleReader,
    guid_reader: StringHandleReader,
}

impl TableReader for ModuleReader {
    type Item = Module;
    const INDEX: TableIndex = TableIndex::Module;

    fn new(sizes: &MetadataSizes) -> ModuleReader {
        ModuleReader {
            string_reader: StringHandleReader::new(sizes),
            guid_reader: GuidHandleReader::new(sizes)
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u16>() + 
            self.string_reader.size() + 
            (3 * self.guid_reader.size())
    }
    
    fn read(&self, buf: &[u8]) -> Result<Module, Error> {
        let cursor = Cursor::new(buf);
        Ok(Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: self.string_reader.read(reader, sizes)?,
            mvid: self.guid_reader.read(reader, sizes)?,
            enc_id: self.guid_reader.read(reader, sizes)?,
            enc_base_id: self.guid_reader.read(reader, sizes)?,
        })
    }
}