mod table_index;

pub mod table;
pub mod module;

pub use self::module::Module;
pub use self::table::{Table, TableRow};
pub use self::table_index::{TableIndex, TableMask};
