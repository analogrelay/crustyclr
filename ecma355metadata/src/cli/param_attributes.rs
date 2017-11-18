// We want ParamAttributes to use the same names as in the ECMA spec, which are PascalCased, not UPPER_SNAKE_CASE
#![allow(non_upper_case_globals)]

bitflags! {
    pub struct ParamAttributes : u16 {
        const In = 0x0001;
        const Out = 0x0002;
        const Optional = 0x0010;
        const HasDefault = 0x1000;
        const HasFieldMarshal = 0x2000;
    }
}

impl_display_via_debug!(ParamAttributes);