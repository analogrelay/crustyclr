use cli::signatures::{TypeReference, CustomModifier};

pub struct RetType {
    pub modifiers: Vec<CustomModifier>,
    pub type_reference: TypeReference,
}