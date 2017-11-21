use std::io::Read;

use cli::tables::TableHandle;
use cli::signatures::utils::{read_type_def_or_ref_spec_encoded, read_compressed_u32};

use error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct CustomModifier {
    required: bool,
    modifier_type: TableHandle,
}

impl CustomModifier {
    pub fn new(required: bool, modifier_type: TableHandle) -> CustomModifier {
        CustomModifier {
            required,
            modifier_type,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<CustomModifier, Error> {
        let required = match read_compressed_u32(reader)? {
            0x20 => false,
            0x1F => true,
            _ => {
                return Err(Error::InvalidMetadata(
                    "Invalid CMOD value for Custom Modifier.",
                ))
            }
        };
        Ok(CustomModifier {
            required,
            modifier_type: read_type_def_or_ref_spec_encoded(reader)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    use cli::tables::{TableHandle, TableIndex};

    #[test]
    pub fn optional() {
        let mut buf = Cursor::new([0x20, 0x42]);
        let modifier = CustomModifier::read(&mut buf).unwrap();
        assert_eq!(
            CustomModifier::new(false, TableHandle::new(0x10, TableIndex::TypeSpec)),
            modifier
        );
    }

    #[test]
    pub fn required() {
        let mut buf = Cursor::new([0x1F, 0x42]);
        let modifier = CustomModifier::read(&mut buf).unwrap();
        assert_eq!(
            CustomModifier::new(true, TableHandle::new(0x10, TableIndex::TypeSpec)),
            modifier
        );
    }
}
