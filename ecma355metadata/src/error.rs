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
}

impl From<::std::io::Error> for Error {
    fn from(e: ::std::io::Error) -> Error {
        Error::IoError(e)
    }
}
