use std::mem;
use std::io::Read;

use cli::tables::{TableHandle, TableIndex};

use error::Error;

// Utilities for reading, used by sub-classes
pub fn read_type_def_or_ref_spec_encoded<R: Read>(reader: &mut R) -> Result<TableHandle, Error> {
    let val = read_compressed_u32(reader)?;

    // Determine the table and index
    let tag = val & 0x03;
    let index = val >> 2;
    let table = match tag {
        0x00 => TableIndex::TypeDef,
        0x01 => TableIndex::TypeRef,
        0x02 => TableIndex::TypeSpec,
        _ => return Err(Error::InvalidMetadata("Invalid TypeDefOrRefSpecEncoded value. Tag value is out of range."))
    };
    Ok(TableHandle::new(index as usize, table))
}


// From: https://source.dot.net/#System.Reflection.Metadata/System/Reflection/Metadata/BlobReader.cs,494
pub fn read_compressed_u32<R: Read>(reader: &mut R) -> Result<u32, Error> {
    Ok(read_compressed_u32_helper(reader)?.0)
}

pub fn read_compressed_i32<R: Read>(reader: &mut R) -> Result<i32, Error> {
    let (mut val, bytes) = read_compressed_u32_helper(reader)?;
    let sign_extend = (val & 0x1) != 0;
    val >>= 1;

    if sign_extend {
        match bytes {
            1 => Ok(unsafe { mem::transmute(val | 0xffffffc0) } ),
            2 => Ok(unsafe { mem::transmute(val | 0xffffe000) } ),
            4 => Ok(unsafe { mem::transmute(val | 0xf0000000) } ),
            _ => panic!("Unexpected compressed integer size"),
        }
    }
    else {
        Ok(val as i32)
    }
}

fn read_compressed_u32_helper<R: Read>(reader: &mut R) -> Result<(u32, usize), Error> {
    let mut buf = [0u8; 4];
    reader.read_exact(&mut buf[0..1])?;
    if buf[0] & 0x80 == 0 {
        // 1-byte number
        Ok(((buf[0] & 0x7F) as u32, 1))
    } else if buf[0] & 0x40 == 0 {
        // 2-byte number
        reader.read_exact(&mut buf[1..2])?;
        let val = ((buf[0] & 0x3F) as u32) << 8 |
            buf[1] as u32;
        Ok((val, 2))
    } else {
        // 4-byte number
        reader.read_exact(&mut buf[1..4])?;
        let val =
            ((buf[0] & 0x1F) as u32) << 24 |
            (buf[1] as u32) << 16 |
            (buf[2] as u32) << 8 |
            (buf[3] as u32);
        Ok((val, 4))
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    macro_rules! read_compressed_u32_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = Cursor::new($encoded);
                    assert_eq!($val, read_compressed_u32(&mut buf).unwrap());
                }
            )*
        };
    }

    macro_rules! read_compressed_i32_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
            $(
                #[test]
                pub fn $name() {
                    let mut buf = Cursor::new($encoded);
                    assert_eq!($val, read_compressed_i32(&mut buf).unwrap());
                }
            )*
        };
    }

    read_compressed_u32_tests!{
        u8_0x03([0x03], 0x03);
        u8_0x7f([0x7F], 0x7F);
        u16_0x80([0x80, 0x80], 0x80);
        u16_0x2e57([0xAE, 0x57], 0x2E57);
        u16_0x3fff([0xBF, 0xFF], 0x3FFF);
        u32_0x4000([0xC0, 0x00, 0x40, 0x00], 0x4000);
        u32_0x1fff_ffff([0xDF, 0xFF, 0xFF, 0xFF], 0x1FFF_FFFF);
    }

    read_compressed_i32_tests!{
        u8_pos_3([0x06], 3);
        u8_neg_3([0x7B], -3);
        u16_pos_64([0x80, 0x80], 64);
        u8_neg_64([0x01], -64);
        u32_pos_8192([0xC0, 0x00, 0x40, 0x00], 8192);
        u16_neg_8192([0x80, 0x01], -8192);
        u32_pos_2pow28([0xDF, 0xFF, 0xFF, 0xFE], 268435455);
        u32_neg_2pow28([0xC0, 0x00, 0x00, 0x01], -268435456);
    }

    #[test]
    pub fn type_def_or_ref_spec_encoded() {
        let mut buf = Cursor::new([0x49]);
        assert_eq!(TableHandle::new(0x12, TableIndex::TypeRef), read_type_def_or_ref_spec_encoded(&mut buf).unwrap());
    }

    #[test]
    pub fn type_def_or_ref_spec_encoded_large() {
        let mut buf = Cursor::new([0xC0, 0x48, 0xD1, 0x5A]);
        assert_eq!(TableHandle::new(0x123456, TableIndex::TypeSpec), read_type_def_or_ref_spec_encoded(&mut buf).unwrap());
    }
}