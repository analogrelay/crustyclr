mod module;
mod type_ref;
mod table_index;

pub mod table;

pub use self::module::Module;
pub use self::type_ref::TypeRef;
pub use self::table::{Table, TableRow};
pub use self::table_index::{TableIndex, TableMask};
