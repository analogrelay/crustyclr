use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidHeap, HeapSizes, MetadataSizes, StringHeap};
use error::Error;
use Guid;

pub trait HeapRef: Sized {
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

pub struct StringRef(usize);

impl HeapRef for StringRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

    fn new(index: usize) -> StringRef {
        StringRef(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}

impl StringRef {
    pub fn get<'a>(&self, heap: &'a StringHeap) -> Option<&'a [u8]> {
        heap.get(self.0)
    }
}

pub struct GuidRef(usize);

impl HeapRef for GuidRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_GUIDS;

    fn new(index: usize) -> GuidRef {
        GuidRef(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}

impl GuidRef {
    pub fn get<'a>(&self, heap: &'a GuidHeap) -> Option<&'a Guid> {
        heap.get(self.0)
    }
}

pub struct BlobRef(usize);

impl HeapRef for BlobRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_BLOBS;

    fn new(index: usize) -> BlobRef {
        BlobRef(index)
    }

    fn index(&self) -> usize {
        self.0
    }
}
