extern crate byteorder;

#[macro_use]
extern crate bitflags;

mod pe_reader;
mod metadata_reader;
mod error;

/// Contains raw data structures from PE/CIL files.
pub mod format;

pub use pe_reader::PeReader;
pub use metadata_reader::MetadataReader;
pub use error::Error;
