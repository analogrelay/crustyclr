const CONV_OR_KIND_MASK: u8 = 0x0F;

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SignatureCallingConvention {
    Default = 0x00,
    CDecl = 0x01,
    StdCall = 0x02,
    ThisCall = 0x03,
    FastCall = 0x04,
    VarArgs = 0x05,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SignatureKind {
    Method = 0x00,
    Field = 0x06,
    LocalVariables = 0x07,
    Property = 0x08,
    MethodSpecification = 0x0A,
}

bitflags! {
    pub struct SignatureAttributes : u8 {
        const GENERIC = 0x10;
        const INSTANCE = 0x20;
        const EXPLICIT_THIS = 0x40;
    }
}

pub struct SignatureHeader {}
