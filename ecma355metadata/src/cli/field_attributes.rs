// We want FieldAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::mem;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct FieldAttributes(u16);

impl FieldAttributes {
    pub fn new(value: u16) -> FieldAttributes {
        FieldAttributes(value)
    }

    pub fn access(self) -> FieldAccess {
        unsafe {
            mem::transmute((self.0 & FieldAccess::MASK) >> FieldAccess::SHIFT)
        }
    }

    pub fn flags(self) -> FieldFlags {
        FieldFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl ::std::fmt::Display for FieldAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", self.access())?;
        if !self.flags().is_empty() {
            write!(f, " [{}]", self.flags())?;
        }
        Ok(())
    }
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum FieldAccess {
    CompilerControlled = 0,
    Private = 1,
    FamANDAssem = 2,
    Assembly = 3,
    Family = 4,
    FamORAssem = 5,
    Public = 6,
}
impl_display_via_debug!(FieldAccess);

impl FieldAccess {
    const MASK: u16 = 0x07;
    const SHIFT: u16 = 0;
}

const FLAGS_MASK: u16 = !(FieldAccess::MASK);

bitflags! {
    pub struct FieldFlags : u16 {
        const Static = 0x0010;
        const InitOnly = 0x0020;
        const Literal = 0x0040;
        const NotSerialized = 0x0080;
        const HasFieldRVA = 0x0100;
        const SpecialName = 0x0200;
        const RTSpecialName = 0x0400;
        const HasFieldMarshal = 0x1000;
        const HasDefault = 0x8000;
    }
}
impl_display_via_debug!(FieldFlags);