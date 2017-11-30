use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::tables::{Table, TableDecoder, TableIndex};
use cli::{BlobHandle, BlobHandleReader, FieldAttributes, MetadataSizes, StringHandle,
          StringHandleReader};

use error::Error;

pub struct Field {
    pub flags: FieldAttributes,
    pub name: StringHandle,
    pub signature: BlobHandle,
}

impl Table for Field {
    type Decoder = FieldDecoder;
    const INDEX: TableIndex = TableIndex::Field;
}

pub struct FieldDecoder {
    count: usize,
    string_reader: StringHandleReader,
    blob_reader: BlobHandleReader,
}

impl<R: Read> TableDecoder<R> for FieldDecoder {
    type Item = Field;

    fn new(sizes: &MetadataSizes) -> FieldDecoder {
        FieldDecoder {
            count: sizes.row_count(Self::Item::INDEX),
            string_reader: StringHandleReader::new(sizes),
            blob_reader: BlobHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u16>() + self.string_reader.size() + self.blob_reader.size()
    }

    fn row_count(&self) -> usize {
        self.count
    }

    fn decode(&self, buf: &mut R) -> Result<Field, Error> {
        Ok(Field {
            flags: FieldAttributes::new(buf.read_u16::<LittleEndian>()?),
            name: self.string_reader.read(buf)?,
            signature: self.blob_reader.read(buf)?,
        })
    }
}
