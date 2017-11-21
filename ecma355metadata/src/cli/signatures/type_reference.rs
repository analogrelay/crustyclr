use std::io::Read;

use byteorder::ReadBytesExt;

use cli::tables::TableHandle;
use cli::signatures::CustomModifier;
use cli::signatures::utils::{read_compressed_u32, read_type_def_or_ref_spec_encoded};

use error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct ArrayShape {
    pub rank: u32,
    pub sizes: Vec<u32>,
    pub lo_bounds: Vec<u32>,
}

impl ArrayShape {
    pub fn new(rank: u32, sizes: Vec<u32>, lo_bounds: Vec<u32>) -> ArrayShape {
        ArrayShape {
            rank,
            sizes,
            lo_bounds,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<ArrayShape, Error> {
        let rank = read_compressed_u32(reader)?;
        let num_sizes = read_compressed_u32(reader)?;
        let mut sizes = Vec::with_capacity(num_sizes as usize);
        for _ in 0..num_sizes {
            sizes.push(read_compressed_u32(reader)?);
        }
        let num_lo_bounds = read_compressed_u32(reader)?;
        let mut lo_bounds = Vec::with_capacity(num_lo_bounds as usize);
        for _ in 0..num_lo_bounds {
            lo_bounds.push(read_compressed_u32(reader)?);
        }

        Ok(ArrayShape::new(rank, sizes, lo_bounds))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GenericInstType {
    Class,
    ValueType,
}

#[derive(Debug, PartialEq, Eq)]
pub enum FnPtrSig {
    MethodDef,
    MethodRef
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeReference {
    End,
    Void,
    Boolean,
    Char,
    I1,
    U1,
    I2,
    U2,
    I4,
    U4,
    I8,
    U8,
    R4,
    R8,
    String,
    Ptr(Vec<CustomModifier>, Box<TypeReference>),
    ByRef(Box<TypeReference>),
    ValueType(TableHandle),
    Class(TableHandle),
    Var(u32),
    Array(Box<TypeReference>, ArrayShape),
    GenericInst(GenericInstType, TableHandle, Vec<TypeReference>),
    TypedByRef,
    I,
    U,
    FnPtr(FnPtrSig),
    Object,
    SzArray(Vec<CustomModifier>, Box<TypeReference>),
    MVar(u32),
}

impl TypeReference {
    pub fn read<R: Read>(reader: &mut R) -> Result<TypeReference, Error> {
        read_helper(read_compressed_u32(reader)?, reader)
    }
}

fn read_helper<R: Read>(discriminator: u32, reader: &mut R) -> Result<TypeReference, Error> {
    match discriminator {
        0x00 => Ok(TypeReference::End),
        0x01 => Ok(TypeReference::Void),
        0x02 => Ok(TypeReference::Boolean),
        0x03 => Ok(TypeReference::Char),
        0x04 => Ok(TypeReference::I1),
        0x05 => Ok(TypeReference::U1),
        0x06 => Ok(TypeReference::I2),
        0x07 => Ok(TypeReference::U2),
        0x08 => Ok(TypeReference::I4),
        0x09 => Ok(TypeReference::U4),
        0x0A => Ok(TypeReference::I8),
        0x0B => Ok(TypeReference::U8),
        0x0C => Ok(TypeReference::R4),
        0x0D => Ok(TypeReference::R8),
        0x0E => Ok(TypeReference::String),
        0x0F => {
            // Ptr
            let (mods, typ) = read_modifiers_and_type(reader)?;
            Ok(TypeReference::Ptr(mods, Box::new(typ)))
        },
        0x10 => Ok(TypeReference::ByRef(Box::new(TypeReference::read(reader)?))),
        0x11 => {
            // ValueType
            let typ = read_type_def_or_ref_spec_encoded(reader)?;
            Ok(TypeReference::ValueType(typ))
        },
        0x12 => {
            // Class
            let typ = read_type_def_or_ref_spec_encoded(reader)?;
            Ok(TypeReference::Class(typ))
        },
        0x13 => Ok(TypeReference::Var(read_compressed_u32(reader)?)),
        0x14 => {
            // Array
            let element_type = TypeReference::read(reader)?;
            let shape = ArrayShape::read(reader)?;
            Ok(TypeReference::Array(Box::new(element_type), shape))
        },
        0x15 => {
            // GenericInst
            let inst_type = match read_compressed_u32(reader)? {
                0x11 => GenericInstType::ValueType,
                0x12 => GenericInstType::Class,
                _ => return Err(Error::InvalidMetadata("Invalid value following ELEMENT_TYPE_GENERIC_INST token."))
            };
            let typ = read_type_def_or_ref_spec_encoded(reader)?;
            let arg_count = read_compressed_u32(reader)?;
            let mut args = Vec::with_capacity(arg_count as usize);
            for _ in 0..arg_count {
                args.push(TypeReference::read(reader)?);
            }
            Ok(TypeReference::GenericInst(inst_type, typ, args))
        },
        0x16 => Ok(TypeReference::TypedByRef),
        0x18 => Ok(TypeReference::I),
        0x19 => Ok(TypeReference::U),
        0x1B => unimplemented!(), // FnPtr
        0x1C => Ok(TypeReference::Object),
        0x1D => {
            // SzArray
            let (mods, typ) = read_modifiers_and_type(reader)?;
            Ok(TypeReference::SzArray(mods, Box::new(typ)))
        }
        0x1E => Ok(TypeReference::MVar(read_compressed_u32(reader)?)),
        x => Err(Error::UnknownTypeCode(x)),
    }
}

fn read_modifiers_and_type<R: Read>(reader: &mut R) -> Result<(Vec<CustomModifier>, TypeReference), Error> {
    let mut cur = read_compressed_u32(reader)?;
    let mut mods = Vec::new();
    while cur == 0x20 || cur == 0x1F {
        let required = match cur {
            0x20 => false,
            0x1F => true,
            _ => {
                return Err(Error::InvalidMetadata(
                    "Invalid CMOD value for Custom Modifier.",
                ))
            }
        };
        mods.push(CustomModifier::new(required, read_type_def_or_ref_spec_encoded(reader)?));
        cur = read_compressed_u32(reader)?;
    }
    let typ = read_helper(cur, reader)?;
    Ok((mods, typ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use cli::tables::{TableIndex, TableHandle};

    macro_rules! type_parse_tests {
        ($($name:ident($data:expr, $expected:expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = ::std::io::Cursor::new($data);
                    let typ = TypeReference::read(&mut buf).unwrap();
                    assert_eq!($expected, typ);
                }
            )*
        };
    }

    type_parse_tests! {
        end([0x00], TypeReference::End);
        void([0x01], TypeReference::Void);
        boolean([0x02], TypeReference::Boolean);
        char([0x03], TypeReference::Char);
        i1([0x04], TypeReference::I1);
        u1([0x05], TypeReference::U1);
        i2([0x06], TypeReference::I2);
        u2([0x07], TypeReference::U2);
        i4([0x08], TypeReference::I4);
        u4([0x09], TypeReference::U4);
        i8([0x0A], TypeReference::I8);
        u8([0x0B], TypeReference::U8);
        r4([0x0C], TypeReference::R4);
        r8([0x0D], TypeReference::R8);
        string([0x0E], TypeReference::String);
        byref_object([0x10, 0x1C], TypeReference::ByRef(Box::new(TypeReference::Object)));
        byref_ptr_byref_i8([0x10, 0x0F, 0x10, 0x0A], TypeReference::ByRef(
            Box::new(TypeReference::Ptr(
                vec![], 
                Box::new(TypeReference::ByRef(
                    Box::new(TypeReference::I8)))))));
        typedbyref([0x16], TypeReference::TypedByRef);
        i([0x18], TypeReference::I);
        u([0x19], TypeReference::U);
        object([0x1C], TypeReference::Object);
        array_boolean([0x14, 0x02, 0x01, 0x01, 0x0A, 0x01, 0x00], TypeReference::Array(
            Box::new(TypeReference::Boolean),
            ArrayShape::new(1, vec![10], vec![0]),
        ));
        valuetype([0x11, 0x42], TypeReference::ValueType(TableHandle::new(0x10, TableIndex::TypeSpec)));
        class([0x12, 0x42], TypeReference::Class(TableHandle::new(0x10, TableIndex::TypeSpec)));
        generic_inst_class([0x15, 0x12, 0x42, 0x02, 0x04, 0x05], TypeReference::GenericInst(
            GenericInstType::Class,
            TableHandle::new(0x10, TableIndex::TypeSpec),
            vec![TypeReference::I1, TypeReference::U1]
        ));
        generic_inst_value_type([0x15, 0x11, 0x42, 0x02, 0x04, 0x05], TypeReference::GenericInst(
            GenericInstType::ValueType,
            TableHandle::new(0x10, TableIndex::TypeSpec),
            vec![TypeReference::I1, TypeReference::U1]
        ));
        var([0x13, 0x42], TypeReference::Var(0x42));
        mvar([0x1E, 0x42], TypeReference::MVar(0x42));
        ptr_char([0x0F, 0x1F, 0x42, 0x20, 0x42, 0x03], TypeReference::Ptr(
            vec![
                CustomModifier::new(true, TableHandle::new(0x10, TableIndex::TypeSpec)),
                CustomModifier::new(false, TableHandle::new(0x10, TableIndex::TypeSpec)),
            ],
            Box::new(TypeReference::Char),
        ));
        ptr_ptr_char([0x0F, 0x0F, 0x03], TypeReference::Ptr(
            vec![],
            Box::new(TypeReference::Ptr(
                vec![], 
                Box::new(TypeReference::Char))),
        ));
        szarray_string([0x1D, 0x1F, 0x42, 0x20, 0x42, 0x0E], TypeReference::SzArray(
            vec![
                CustomModifier::new(true, TableHandle::new(0x10, TableIndex::TypeSpec)),
                CustomModifier::new(false, TableHandle::new(0x10, TableIndex::TypeSpec)),
            ],
            Box::new(TypeReference::String)
        ));
    }
}
