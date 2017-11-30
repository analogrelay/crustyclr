extern crate ecma355metadata;

extern crate memmap;
#[macro_use]
extern crate slog;

mod app_context;
mod assembly;
mod runtime;

pub mod error;

pub use app_context::AppContext;
pub use assembly::Assembly;
pub use runtime::{Runtime, RuntimeBuilder};
