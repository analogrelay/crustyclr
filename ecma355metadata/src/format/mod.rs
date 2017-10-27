mod error;
mod pe_header;

pub use self::error::Error;
pub use self::pe_header::PeHeader;

pub type Result<T> = ::std::result::Result<T, Error>;
