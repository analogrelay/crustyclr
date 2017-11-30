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
mod param;
mod table_decoder;
mod table_handle;
mod table_index;
// mod table_stream;
mod table;

pub use self::module::{Module, ModuleDecoder};
pub use self::type_ref::{TypeRef, TypeRefDecoder};
pub use self::type_def::{TypeDef, TypeDefDecoder};
pub use self::field::{Field, FieldDecoder};
pub use self::method_def::{MethodDef, MethodDefDecoder};
pub use self::param::{Param, ParamDecoder};
pub use self::table_decoder::TableDecoder;
pub use self::table_handle::{TableHandle, TableHandleReader};
pub use self::table_index::{TableIndex, TableMask};
// pub use self::table_stream::TableStream;
pub use self::table::Table;
