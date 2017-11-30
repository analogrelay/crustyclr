use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{BlobHandle, BlobHandleReader, MetadataSizes, MethodAttributes, MethodImplAttributes,
          StringHandle, StringHandleReader};
use cli::tables::{Table, TableDecoder, TableHandle, TableHandleReader, TableIndex};

use error::Error;

pub struct MethodDef {
    pub rva: u32,
    pub impl_flags: MethodImplAttributes,
    pub flags: MethodAttributes,
    pub name: StringHandle,
    pub signature: BlobHandle,
    pub params: TableHandle,
}

impl Table for MethodDef {
    type Decoder = MethodDefDecoder;
    const INDEX: TableIndex = TableIndex::MethodDef;
}

pub struct MethodDefDecoder {
    count: usize,
    string_reader: StringHandleReader,
    blob_reader: BlobHandleReader,
    params_reader: TableHandleReader,
}

impl<R: Read> TableDecoder<R> for MethodDefDecoder {
    type Item = MethodDef;

    fn new(sizes: &MetadataSizes) -> MethodDefDecoder {
        MethodDefDecoder {
            count: sizes.row_count(Self::Item::INDEX),
            string_reader: StringHandleReader::new(sizes),
            blob_reader: BlobHandleReader::new(sizes),
            params_reader: index_reader!(sizes, TableIndex::Param),
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u32>() + (2 * size_of::<u16>()) + self.string_reader.size()
            + self.blob_reader.size() + self.params_reader.size()
    }

    fn row_count(&self) -> usize {
        self.count
    }

    fn decode(&self, buf: &mut R) -> Result<MethodDef, Error> {
        Ok(MethodDef {
            rva: buf.read_u32::<LittleEndian>()?,
            impl_flags: MethodImplAttributes::new(buf.read_u16::<LittleEndian>()?),
            flags: MethodAttributes::new(buf.read_u16::<LittleEndian>()?),
            name: self.string_reader.read(buf)?,
            signature: self.blob_reader.read(buf)?,
            params: self.params_reader.read(buf)?,
        })
    }
}
