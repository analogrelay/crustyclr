use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidHandle, GuidHandleReader, MetadataSizes, StringHandle, StringHandleReader};
use cli::tables::{Table, TableDecoder, TableIndex};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringHandle,
    pub mvid: GuidHandle,
    pub enc_id: GuidHandle,
    pub enc_base_id: GuidHandle,
}

impl Table for Module {
    type Decoder = ModuleDecoder;
    const INDEX: TableIndex = TableIndex::Module;
}

pub struct ModuleDecoder {
    count: usize,
    string_reader: StringHandleReader,
    guid_reader: GuidHandleReader,
}

impl<R: Read> TableDecoder<R> for ModuleDecoder {
    type Item = Module;

    fn new(sizes: &MetadataSizes) -> ModuleDecoder {
        ModuleDecoder {
            count: sizes.row_count(Self::Item::INDEX),
            string_reader: StringHandleReader::new(sizes),
            guid_reader: GuidHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u16>() + self.string_reader.size() + (3 * self.guid_reader.size())
    }

    fn row_count(&self) -> usize {
        self.count
    }

    fn decode(&self, mut buf: &mut R) -> Result<Module, Error> {
        Ok(Module {
            generation: buf.read_u16::<LittleEndian>()?,
            name: self.string_reader.read(buf)?,
            mvid: self.guid_reader.read(buf)?,
            enc_id: self.guid_reader.read(buf)?,
            enc_base_id: self.guid_reader.read(buf)?,
        })
    }
}
