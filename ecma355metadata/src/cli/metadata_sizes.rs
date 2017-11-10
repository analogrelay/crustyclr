use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::tables::{TableIndex, TableMask};
use error::Error;

bitflags! {
    pub struct HeapSizes: u8 {
        const LARGE_STRINGS = 0x01;
        const LARGE_GUIDS = 0x02;
        const LARGE_BLOBS = 0x04;
        const EXTRA_DATA = 0x40;
    }
}

impl ::std::fmt::Display for HeapSizes {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{:?}", self)
    }
}

pub struct MetadataSizes {
    heap_sizes: HeapSizes,
    row_counts: [usize; TableIndex::MAX],
}

impl MetadataSizes {
    pub fn read<A: Read>(buf: &mut A) -> Result<MetadataSizes, Error> {
        // Skip reserved value, and version numbers
        buf.read_u32::<LittleEndian>()?;
        buf.read_u8()?;
        buf.read_u8()?;
        let heap_sizes = HeapSizes::from_bits_truncate(buf.read_u8()?);

        // Skip reserved value
        buf.read_u8()?;

        // Read valid and sorted vectors
        let valid_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);
        let _sorted_mask = TableMask::from_bits_truncate(buf.read_u64::<LittleEndian>()?);

        // Load row counts
        let mut row_counts = [0; TableIndex::MAX];
        for idx in TableIndex::each() {
            if valid_mask.has_table(idx) {
                let size = buf.read_u32::<LittleEndian>()?;
                row_counts[idx as usize] = size as usize;
            }
        }

        Ok(MetadataSizes {
            heap_sizes: heap_sizes,
            row_counts: row_counts,
        })
    }

    pub fn heap_sizes(&self) -> HeapSizes {
        self.heap_sizes
    }

    pub fn row_count(&self, idx: TableIndex) -> usize {
        self.row_counts[idx as usize]
    }
}
