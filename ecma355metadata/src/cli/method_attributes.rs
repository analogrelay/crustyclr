// We want MethodAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

use std::mem;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MethodAttributes(u16);

impl MethodAttributes {
    pub fn new(value: u16) -> MethodAttributes {
        MethodAttributes(value)
    }

    pub fn access(self) -> MethodAccess {
        unsafe {
            mem::transmute((self.0 & MethodAccess::MASK) >> MethodAccess::SHIFT)
        }
    }

    pub fn vtable_layout(self) -> MethodVTableLayout {
        unsafe {
            mem::transmute((self.0 & MethodVTableLayout::MASK) >> MethodVTableLayout::SHIFT)
        }
    }

    pub fn flags(self) -> MethodFlags {
        MethodFlags::from_bits_truncate(self.0 & FLAGS_MASK)
    }
}

impl ::std::fmt::Display for MethodAttributes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{} {}", self.access(), self.vtable_layout())?;
        if !self.flags().is_empty() {
            write!(f, " [{}]", self.flags())?;
        }
        Ok(())
    }
}

MethodAccess and FieldAccess are the same value! They should be shared

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum MethodAccess {
    CompilerControlled = 0,
    Private = 1,
    FamANDAssem = 2,
    Assembly = 3,
    Family = 4,
    FamORAssem = 5,
    Public = 6,
}
impl_display_via_debug!(MethodAccess);

impl MethodAccess {
    const MASK: u16 = 0x07;
    const SHIFT: u16 = 0;
}

#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum MethodVTableLayout {
    ReuseSlot = 0,
    NewSlot = 1,
}
impl_display_via_debug!(MethodVTableLayout);

impl MethodVTableLayout {
    const MASK: u16 = 0x0100;
    const SHIFT: u16 = 8;
}

const FLAGS_MASK: u16 = !(MethodAccess::MASK | MethodVTableLayout::MASK);

bitflags! {
    pub struct MethodFlags : u16 {
        const UnmanagedExport = 0x0008;
        const Static = 0x0010;
        const Final = 0x0020;
        const Virtual = 0x0040;
        const HideBySig = 0x0080;
        const Strict = 0x0200;
        const Abstract = 0x0400;
        const SpecialName = 0x0800;
        const RTSpecialName = 0x1000;
        const PInvokeImpl = 0x2000;
        const HasSecurity = 0x4000;
        const RequireSecObject = 0x8000;
    }
}
impl_display_via_debug!(MethodFlags);