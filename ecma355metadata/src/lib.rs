extern crate byteorder;

mod portable_executable;
mod error;

/// Contains raw data structures from PE/CIL files.
pub mod format;

pub use portable_executable::PortableExecutable;
pub use error::Error;
