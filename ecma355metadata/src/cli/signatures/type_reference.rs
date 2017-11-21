use std::io::Read;

use byteorder::ReadBytesExt;

use cli::tables::TableHandle;
use cli::signatures::utils::read_compressed_u32;

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
        for i in 0..num_sizes {
            sizes.push(read_compressed_u32(reader)?);
        }
        let num_lo_bounds = read_compressed_u32(reader)?;
        let mut lo_bounds = Vec::with_capacity(num_lo_bounds as usize);
        for i in 0..num_lo_bounds {
            lo_bounds.push(read_compressed_u32(reader)?);
        }

        Ok(ArrayShape::new(rank, sizes, lo_bounds))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TypeReference {
    Void,
    Boolean,
    Char,
    I1,
    I2,
    I4,
    I8,
    U1,
    U2,
    U4,
    U8,
    R4,
    R8,
    I,
    U,
    Array(Box<TypeReference>, ArrayShape),
    Class(TableHandle),
    FnPtrMethodDef,
    FnPtrMethodRef,
    GenericInstClass,
    GenericInstValueType,
    MVar(u32),
    Object,
    Ptr(Box<TypeReference>),
    String,
    SzArray,
    ValueType,
    Var(u32),
}

impl TypeReference {
    pub fn read<R: Read>(reader: &mut R) -> Result<TypeReference, Error> {
        let discriminator = reader.read_u8()?;
        match discriminator {
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
            0x14 => {
                // Array
                let element_type = TypeReference::read(reader)?;
                let shape = ArrayShape::read(reader)?;
                Ok(TypeReference::Array(Box::new(element_type), shape))
            }
            0x18 => Ok(TypeReference::I),
            0x19 => Ok(TypeReference::U),
            0x1C => Ok(TypeReference::Object),
            _ => Err(Error::UnknownTypeCode),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        object([0x0E], TypeReference::String);
        i([0x18], TypeReference::I);
        u([0x19], TypeReference::U);
        string([0x1C], TypeReference::Object);
        array_boolean([0x14, 0x02, 0x01, 0x01, 0x0A, 0x01, 0x00], TypeReference::Array(
            Box::new(TypeReference::Boolean),
            ArrayShape::new(1, vec![10], vec![0]),
        ));
    }
}
