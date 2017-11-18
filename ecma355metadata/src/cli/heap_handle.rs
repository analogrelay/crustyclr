use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataSizes, LARGE_INDEX_SIZE, SMALL_INDEX_SIZE};
use error::Error;

macro_rules! handle_type {
    ($name:ident, $reader:ident, $size_flag:expr) => {
        #[derive(Debug, Eq, PartialEq, Clone, Copy)]
        pub struct $name(usize);

        impl $name {
            const SIZE_FLAG: HeapSizes = $size_flag;

            pub fn new(index: usize) -> $name {
                $name(index)
            }

            pub fn read<R: Read>(reader: &mut R, large: bool) -> Result<$name, Error> {
                Ok($name(read_heap_handle(reader, large)?))
            }

            pub fn index(&self) -> usize {
                self.0
            }
        }

        pub struct $reader(bool);

        impl $reader {
            pub fn new(sizes: &MetadataSizes) -> $reader {
                $reader(sizes.heap_sizes().contains($name::SIZE_FLAG))
            }

            pub fn size(&self) -> usize {
                if self.0 {
                    LARGE_INDEX_SIZE
                } else {
                    SMALL_INDEX_SIZE
                }
            }

            pub fn read<R: Read>(&self, reader: &mut R) -> Result<$name, Error> {
                $name::read(reader, self.0)
            }
        }
    };
}

handle_type!(StringHandle, StringHandleReader, HeapSizes::LARGE_STRINGS);
handle_type!(GuidHandle, GuidHandleReader, HeapSizes::LARGE_GUIDS);
handle_type!(BlobHandle, BlobHandleReader, HeapSizes::LARGE_BLOBS);

fn read_heap_handle<R: Read>(reader: &mut R, large: bool) -> Result<usize, Error> {
    if large {
        Ok(reader.read_u32::<LittleEndian>()? as usize)
    } else {
        Ok(reader.read_u16::<LittleEndian>()? as usize)
    }
}
