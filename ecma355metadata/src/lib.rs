extern crate byteorder;

mod pe_reader;
mod error;

/// Contains raw data structures from PE/CIL files.
pub mod format;

pub use pe_reader::PeReader;
pub use error::Error;
