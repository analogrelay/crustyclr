use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataSizes, SMALL_INDEX_SIZE, LARGE_INDEX_SIZE};
use error::Error;

macro_rules! handle_type {
    ($name:ident, $reader:ident, $size_flag:expr) => {
        pub struct $name(usize);

        impl $name {
            const SIZE_FLAG: HeapSizes = $size_flag;

            fn new(index: usize) -> $name {
                $name(index)
            }

            fn read<R: Read>(reader: &mut R, large: bool) -> Result<$name, Error> {
                $name(read_heap_handle(reader, large)?)
            }

            fn index(&self) -> usize {
                self.0
            }
        }

        pub struct $reader(bool);

        impl $reader {
            pub fn new(sizes: &MetadataSizes) -> $reader {
                $reader(sizes.heap_sizes().contains($size_flag))
            }

            pub fn size(&self) -> usize {
                if self.0 {
                    LARGE_INDEX_SIZE   
                } else {
                    SMALL_INDEX_SIZE
                }
            }

            pub fn read<R: Read>(reader: &mut R) -> Result<$name, Error> {
                $name::read(reader, self.0)
            }
        }
    };
}

handle_type!(StringHandle, StringHandleReader, HeapSizes::LARGE_STRINGS);
handle_type!(GuidHandle, GuidHandleReader, HeapSizes::LARGE_GUIDS);
handle_type!(BlobHandle, BlobHandleReader, HeapSizes::LARGE_BLOBS);

fn read_heap_handle<R>(reader: &mut R, large: bool) -> Result<usize, Error> {
    if large {
        reader.read_u32::<LittleEndian>()? as usize
    } else {
        reader.read_u16::<LittleEndian>()? as usize
    }
}
