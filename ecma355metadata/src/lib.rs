extern crate byteorder;

#[macro_use]
extern crate bitflags;

mod error;
mod utils;
mod guid;

/// Contains CLI metadata structures
pub mod cli;

/// Contains PE structures
pub mod pe;

pub use error::Error;

pub use pe::PeImage;
pub use cli::MetadataReader;
pub use guid::Guid;