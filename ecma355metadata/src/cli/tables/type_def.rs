use std::io::Cursor;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{MetadataSizes, StringHandle, StringHandleReader, TypeAttributes};
use cli::tables::{TableHandle, TableIndex, TableMask, TableReader, TableHandleReader};
use error::Error;

pub struct TypeDef {
    pub flags: TypeAttributes,
    pub type_name: StringHandle,
    pub type_namespace: StringHandle,
    pub extends: TableHandle,
    pub field_list: TableHandle,
    pub method_list: TableHandle,
}

pub struct TypeDefReader {
    type_def_or_ref_reader: TableHandleReader,
    field_list_reader: TableHandleReader,
    method_list_reader: TableHandleReader,
    string_reader: StringHandleReader,
}

impl TableReader for TypeDefReader {
    type Item = TypeDef;
    const INDEX: TableIndex = TableIndex::TypeDef;

    fn new(sizes: &MetadataSizes) -> TypeDefReader {
        TypeDefReader {
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

    fn read(&self, buf: &[u8]) -> Result<TypeDef, Error> {
        let mut cursor = Cursor::new(buf);

        Ok(TypeDef {
            flags: TypeAttributes::new(cursor.read_u32::<LittleEndian>()?),
            type_name: self.string_reader.read(&mut cursor)?,
            type_namespace: self.string_reader.read(&mut cursor)?,
            extends: self.type_def_or_ref_reader.read(&mut cursor)?,
            field_list: self.field_list_reader.read(&mut cursor)?,
            method_list: self.method_list_reader.read(&mut cursor)?,
        })
    }
}
