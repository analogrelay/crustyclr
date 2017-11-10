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
