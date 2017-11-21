mod signature_header;

pub use self::signature_header::SignatureHeader;

use std::mem;
use std::io::Read;

use error::Error;

// Utilities for reading, used by sub-classes
// From: https://source.dot.net/#System.Reflection.Metadata/System/Reflection/Metadata/BlobReader.cs,494
fn read_compressed_usize<R: Read>(reader: &mut R) -> Result<u32, Error> {
    Ok(read_compressed_usize_helper(reader)?.0)
}

fn read_compressed_isize<R: Read>(reader: &mut R) -> Result<i32, Error> {
    let (mut val, bytes) = read_compressed_usize_helper(reader)?;
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

fn read_compressed_usize_helper<R: Read>(reader: &mut R) -> Result<(u32, usize), Error> {
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

    macro_rules! read_compressed_usize_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
           $(
               #[test]
               pub fn $name() {
                   let mut buf = ::std::io::Cursor::new($encoded);
                   assert_eq!($val, read_compressed_usize(&mut buf).unwrap());
               }
           )*
        };
    }

    macro_rules! read_compressed_isize_tests {
        ($($name: ident($encoded: expr, $val: expr);)*) => {
           $(
               #[test]
               pub fn $name() {
                   let mut buf = ::std::io::Cursor::new($encoded);
                   assert_eq!($val, read_compressed_isize(&mut buf).unwrap());
               }
           )*
        };
    }

    read_compressed_usize_tests!{
        u8_0x03([0x03], 0x03);
        u8_0x7f([0x7F], 0x7F);
        u16_0x80([0x80, 0x80], 0x80);
        u16_0x2e57([0xAE, 0x57], 0x2E57);
        u16_0x3fff([0xBF, 0xFF], 0x3FFF);
        u32_0x4000([0xC0, 0x00, 0x40, 0x00], 0x4000);
        u32_0x1fff_ffff([0xDF, 0xFF, 0xFF, 0xFF], 0x1FFF_FFFF);
    }

    read_compressed_isize_tests!{
        u8_pos_3([0x06], 3);
        u8_neg_3([0x7B], -3);
        u16_pos_64([0x80, 0x80], 64);
        u8_neg_64([0x01], -64);
        u32_pos_8192([0xC0, 0x00, 0x40, 0x00], 8192);
        u16_neg_8192([0x80, 0x01], -8192);
        u32_pos_2pow28([0xDF, 0xFF, 0xFF, 0xFE], 268435455);
        u32_neg_2pow28([0xC0, 0x00, 0x00, 0x01], -268435456);
    }
}
