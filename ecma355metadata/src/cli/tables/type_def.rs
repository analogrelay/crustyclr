use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{MetadataSizes, StringHandle, StringHandleReader, TypeAttributes};
use cli::tables::{TableHandle, TableIndex, TableMask, TableDecoder, TableHandleReader};
use error::Error;

pub struct TypeDef {
    pub flags: TypeAttributes,
    pub type_name: StringHandle,
    pub type_namespace: StringHandle,
    pub extends: TableHandle,
    pub field_list: TableHandle,
    pub method_list: TableHandle,
}

pub struct TypeDefDecoder {
    count: usize,
    type_def_or_ref_reader: TableHandleReader,
    field_list_reader: TableHandleReader,
    method_list_reader: TableHandleReader,
    string_reader: StringHandleReader,
}

impl TableDecoder for TypeDefDecoder {
    type Item = TypeDef;
    const INDEX: TableIndex = TableIndex::TypeDef;

    fn new(sizes: &MetadataSizes) -> TypeDefDecoder {
        TypeDefDecoder {
            count: sizes.row_count(Self::INDEX),
            type_def_or_ref_reader: index_reader!(sizes, 
                0 => TableIndex::TypeDef,
                1 => TableIndex::TypeRef,
                2 => TableIndex::TypeSpec),
            field_list_reader: index_reader!(sizes, TableIndex::Field),
            method_list_reader: index_reader!(sizes, TableIndex::MethodDef),
            string_reader: StringHandleReader::new(sizes),
        }
    }

    fn row_size(&self) -> usize {
        size_of::<u32>() +
            self.string_reader.size() +
            self.string_reader.size() +
            self.type_def_or_ref_reader.size() +
            self.field_list_reader.size() +
            self.method_list_reader.size()
    }

    fn row_count(&self) -> usize {
        self.count
    }

    fn decode(&self, mut buf: &[u8]) -> Result<TypeDef, Error> {
        Ok(TypeDef {
            flags: TypeAttributes::new(buf.read_u32::<LittleEndian>()?),
            type_name: self.string_reader.read(&mut buf)?,
            type_namespace: self.string_reader.read(&mut buf)?,
            extends: self.type_def_or_ref_reader.read(&mut buf)?,
            field_list: self.field_list_reader.read(&mut buf)?,
            method_list: self.method_list_reader.read(&mut buf)?,
        })
    }
}
