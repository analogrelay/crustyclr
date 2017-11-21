use std::io::Read;

use cli::signatures::{CustomModifier, TypeReference};
use cli::signatures::utils;

use error::Error;

#[derive(Debug, PartialEq, Eq)]
pub struct RetType {
    pub modifiers: Vec<CustomModifier>,
    pub type_reference: TypeReference,
}

impl RetType {
    pub fn new(modifiers: Vec<CustomModifier>, type_reference: TypeReference) -> RetType {
        RetType {
            modifiers,
            type_reference,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<RetType, Error> {
        let (mods, typ) = utils::read_modifiers_and_type(reader)?;
        Ok(RetType::new(mods, typ))
    }
}