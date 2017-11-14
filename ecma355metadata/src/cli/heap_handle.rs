use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataSizes};
use error::Error;

pub trait HeapHandle: Sized {
    const SIZE_FLAG: HeapSizes;

    fn new(index: usize) -> Self;
    fn index(&self) -> usize;

    fn size(sizes: &MetadataSizes) -> usize {
        if sizes.heap_sizes().contains(Self::SIZE_FLAG) {
            size_of::<u32>()
        } else {
            size_of::<u16>()
        }
    }

    fn read<R: Read>(reader: &mut R, sizes: &MetadataSizes) -> Result<Self, Error> {
        if sizes.heap_sizes().contains(Self::SIZE_FLAG) {
            Ok(Self::new(reader.read_u32::<LittleEndian>()? as usize))
        } else {
            Ok(Self::new(reader.read_u16::<LittleEndian>()? as usize))
        }
    }
}

pub struct StringHandle(usize);

impl HeapHandle for StringHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

    fn new(index: usize) -> StringHandle {
        StringHandle(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}

pub struct GuidHandle(usize);

impl HeapHandle for GuidHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_GUIDS;

    fn new(index: usize) -> GuidHandle {
        GuidHandle(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}

pub struct BlobHandle(usize);

impl HeapHandle for BlobHandle {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_BLOBS;

    fn new(index: usize) -> BlobHandle {
        BlobHandle(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}
