use std::io;

use ecma355metadata;

#[derive(Debug)]
pub enum Error {
    AssemblyNotFound(String),
    BadImageFormat(ecma355metadata::Error),
    IoError(io::Error),
}

impl From<io::Error> for Error {
    fn from(v: io::Error) -> Error {
        Error::IoError(v)
    }
}

impl From<ecma355metadata::Error> for Error {
    fn from(v: ecma355metadata::Error) -> Error {
        Error::BadImageFormat(v)
    }
}

// Manual implementation because io::Error doesn't implement PartialEq, so we can't derive... but it's
// OK with us if IoError != IoError because this is mostly for testing.
// We don't implement Eq though, because Eq implies `l.eq(r)` will always be `true` for the same `l`, `r`
// but that's not the case (IoError is like NaN)
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::AssemblyNotFound(ref lhs), &Error::AssemblyNotFound(ref rhs)) => lhs.eq(rhs),
            (&Error::BadImageFormat(ref lhs), &Error::BadImageFormat(ref rhs)) => lhs.eq(rhs),
            _ => false, // Type mismatches and IoError are never equal
        }
    }
}