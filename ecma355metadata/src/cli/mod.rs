mod access;
mod cli_header;
mod metadata_header;
mod cli_flags;
mod stream_header;
mod metadata_reader;
mod heap_handle;
mod string_heap;
mod guid_heap;
mod blob_heap;
mod metadata_sizes;
mod type_attributes;
mod field_attributes;
mod method_attributes;
mod method_impl_attributes;
mod param_attributes;

pub mod tables;
pub mod signatures;

pub use self::access::Access;
pub use self::cli_header::CliHeader;
pub use self::metadata_header::MetadataHeader;
pub use self::cli_flags::CliFlags;
pub use self::stream_header::StreamHeader;
pub use self::metadata_reader::MetadataReader;
pub use self::heap_handle::{BlobHandle, BlobHandleReader, GuidHandle, GuidHandleReader,
                            StringHandle, StringHandleReader};
pub use self::guid_heap::GuidHeap;
pub use self::blob_heap::BlobHeap;
pub use self::string_heap::StringHeap;
pub use self::type_attributes::{TypeAttributes, TypeFlags, TypeLayout, TypeSemantics,
                                TypeStringFormat, TypeVisibility};
pub use self::field_attributes::{FieldAttributes, FieldFlags};
pub use self::method_attributes::{MethodAttributes, MethodFlags, MethodVTableLayout};
pub use self::method_impl_attributes::{MethodCodeType, MethodImplAttributes, MethodImplFlags};
pub use self::param_attributes::ParamAttributes;
pub use self::metadata_sizes::{HeapSizes, MetadataSizes, LARGE_INDEX_SIZE, SMALL_INDEX_SIZE,
                               SMALL_TABLE_MAX_SIZE};
