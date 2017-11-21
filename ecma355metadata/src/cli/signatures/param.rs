use std::io::Read;

use cli::signatures::{CustomModifier, TypeReference};
use cli::signatures::utils;

use error::Error;

pub struct Param {
    pub modifiers: Vec<CustomModifier>,
    pub type_reference: TypeReference,
}

impl Param {
    pub fn new(modifiers: Vec<CustomModifier>, type_reference: TypeReference) -> Param {
        Param {
            modifiers,
            type_reference,
        }
    }

    pub fn read<R: Read>(reader: &mut R) -> Result<Param, Error> {
        let (mods, typ) = utils::read_modifiers_and_type(reader)?;
        Ok(Param::new(mods, typ))
    }
}