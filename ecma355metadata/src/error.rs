/// Represents an error that occurs while loading PE/CIL metadata
#[derive(Debug)]
pub enum Error {
    /// Indicates that an I/O error occurred.
    IoError(::std::io::Error),

    /// Indicates that the file has an invalid signature (MS-DOS Signature, PE Signature, etc.).
    InvalidSignature,

    /// Indicates that the file is not a PE file, and thus has no PE header.
    NotAPortableExecutable,

    /// The requested PE data directory was not found.
    DirectoryNotFound,

    /// The requested section was not found.
    SectionNotFound,

    /// The image does not contain a CLI header
    CliHeaderNotFound,

    /// The image contains a string which is not valid UTF-8 or UTF-16
    InvalidStringData,

    /// The requested metadata stream was not found.
    StreamNotFound,

    /// The metadata file is invalid in an unexpected way.
    InvalidMetadata(&'static str),

    /// An invalid heap reference was provided.
    InvalidHeapReference,

    /// The provided table name was not recognized
    UnknownTableName,

    /// The Coded Index data was invalid
    InvalidCodedIndex,

    /// The type code is not recognized
    UnknownTypeCode(u32),
}

// Manual implementation because io::Error doesn't implement PartialEq, so we can't derive... but it's
// OK with us if IoError != IoError because this is mostly for testing.
// We don't implement Eq though, because Eq implies `l.eq(r)` will always be `true` for the same `l`, `r`
// but that's not the case (IoError is like NaN)
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (&Error::InvalidSignature, &Error::InvalidSignature) => true,
            (&Error::NotAPortableExecutable, &Error::NotAPortableExecutable) => true,
            (&Error::DirectoryNotFound, &Error::DirectoryNotFound) => true,
            (&Error::SectionNotFound, &Error::SectionNotFound) => true,
            (&Error::CliHeaderNotFound, &Error::CliHeaderNotFound) => true,
            (&Error::InvalidStringData, &Error::InvalidStringData) => true,
            (&Error::StreamNotFound, &Error::StreamNotFound) => true,
            (&Error::InvalidMetadata(lhs), &Error::InvalidMetadata(rhs)) => lhs.eq(rhs),
            (&Error::InvalidHeapReference, &Error::InvalidHeapReference) => true,
            (&Error::UnknownTableName, &Error::UnknownTableName) => true,
            (&Error::InvalidCodedIndex, &Error::InvalidCodedIndex) => true,
            (&Error::UnknownTypeCode(lhs), &Error::UnknownTypeCode(rhs)) => lhs == rhs,
            _ => false, // Type mismatches and IoError are never equal
        }
    }
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        Error::IoError(e)
    }
}

impl From<::std::str::Utf8Error> for Error {
    fn from(_: ::std::str::Utf8Error) -> Error {
        Error::InvalidStringData
    }
}

impl From<::std::string::FromUtf8Error> for Error {
    fn from(_: ::std::string::FromUtf8Error) -> Error {
        Error::InvalidStringData
    }
}

impl From<::std::string::FromUtf16Error> for Error {
    fn from(_: ::std::string::FromUtf16Error) -> Error {
        Error::InvalidStringData
    }
}
