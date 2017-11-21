use cli::tables::TableHandle;

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
}