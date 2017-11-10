use std::fmt::{Display, Error, Formatter};

#[derive(Clone, Copy)]
pub struct Guid([u8; 16]);

impl Display for Guid {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let &Guid(ref values) = self;
        write!(
            f,
            "{{{:X}{:X}{:X}{:X}-{:X}{:X}-{:X}{:X}-{:X}{:X}-{:X}{:X}{:X}{:X}{:X}{:X}}}",
            values[0],
            values[1],
            values[2],
            values[3],
            values[4],
            values[5],
            values[6],
            values[7],
            values[8],
            values[9],
            values[10],
            values[11],
            values[12],
            values[13],
            values[14],
            values[15]
        )
    }
}
