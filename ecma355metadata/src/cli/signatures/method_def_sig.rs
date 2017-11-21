use std::io::Read;

use byteorder::ReadBytesExt;

use cli::signatures::{SignatureHeader, TypeReference};

use error::Error;

pub struct MethodDefSig {
    pub header: SignatureHeader,
    pub return_type: TypeReference,
    pub required_parameter_count: u32,
    pub generic_parameter_count: u32,
    pub parameter_types: Vec<TypeReference>,
}

impl MethodDefSig {
    pub fn new(
        header: SignatureHeader,
        return_type: TypeReference,
        required_parameter_count: u32,
        generic_parameter_count: u32,
        parameter_types: Vec<TypeReference>,
    ) -> MethodDefSig {
        MethodDefSig {
            header,
            return_type,
            required_parameter_count,
            generic_parameter_count,
            parameter_types,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<MethodDefSig, Error> {
        let _header = SignatureHeader::new(reader.read_u8()?);
        unimplemented!();
    }
}
