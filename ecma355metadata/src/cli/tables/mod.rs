// This macro has to be up here because macros are process top-down and the modules
// referenced here use it.
macro_rules! index_reader {
    ($sizes:expr, $idx:expr) => {
        TableHandleReader::for_simple_index(
            $sizes.index_size($idx) == $crate::cli::LARGE_INDEX_SIZE,
            |_| Some($idx))
    };
    (
        $sizes:expr, $(
            $tag:expr => $idx:expr
        ),*
    ) => {
        {
            let tables = $(
                TableMask::from_index($idx)
            )|+;
            TableHandleReader::for_coded_index(
                $sizes.coded_index_size(tables) == $crate::cli::LARGE_INDEX_SIZE,
                tables,
                |tag| match tag {
                    $(
                        $tag => Some($idx)
                    ),+,
                    _ => None
                })
        }
    }
}

mod module;
mod type_ref;
mod type_def;
mod field;
mod method_def;
mod table;
mod table_index;
mod table_handle;
mod table_reader;
mod table_stream;

pub use self::module::{Module, ModuleReader};
pub use self::type_ref::{TypeRef, TypeRefReader};
pub use self::type_def::{TypeDef, TypeDefReader};
pub use self::field::{Field, FieldReader};
pub use self::method_def::{MethodDef, MethodDefReader};
pub use self::table::Table;
pub use self::table_reader::TableReader;
pub use self::table_index::{TableIndex, TableMask};
pub use self::table_handle::{TableHandle, TableHandleReader};
pub use self::table_stream::TableStream;
