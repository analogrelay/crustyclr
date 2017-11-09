use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::HeapSizes;
use error::Error;

pub trait HeapRef: Sized {
    const SIZE_FLAG: HeapSizes;

    fn new(index: u32) -> Self;

    fn size(heap_sizes: HeapSizes) -> usize {
        if heap_sizes.contains(Self::SIZE_FLAG) {
            size_of::<u32>()
        } else {
            size_of::<u16>()
        }
    }

    fn read<R: Read>(reader: &mut R, heap_sizes: HeapSizes) -> Result<Self, Error> {
        if heap_sizes.contains(Self::SIZE_FLAG) {
            Ok(Self::new(reader.read_u32::<LittleEndian>()?))
        } else {
            Ok(Self::new(reader.read_u16::<LittleEndian>()? as u32))
        }
    }
}

pub struct StringRef(u32);
pub struct GuidRef(u32);
pub struct BlobRef(u32);

impl HeapRef for StringRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

    fn new(index: u32) -> StringRef {
        StringRef(index)
    }
}

impl HeapRef for GuidRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_GUIDS;

    fn new(index: u32) -> GuidRef {
        GuidRef(index)
    }
}

impl HeapRef for BlobRef {
    const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_BLOBS;

    fn new(index: u32) -> BlobRef {
        BlobRef(index)
    }
}
