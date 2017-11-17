use std::mem;

use error::Error;
use Guid;

pub struct GuidHeap<'a> {
    data: Option<&'a [Guid]>,
}

impl<'a> GuidHeap<'a> {
    pub const EMPTY: GuidHeap<'static> = GuidHeap { data: None };

    pub fn new(data: &'a [u8]) -> Result<GuidHeap<'a>, Error> {
        // Make sure the data is a multiple of 16 in length
        if data.len() % mem::size_of::<Guid>() != 0 {
            return Err(Error::InvalidMetadata(
                "GUID stream is not a multiple of 16 bytes in length.",
            ));
        }
        Ok(GuidHeap { data: Some(unsafe { mem::transmute(data) }) })
    }

    pub fn get(&self, idx: usize) -> Option<&Guid> {
        if let Some(data) = self.data {
            // Determine the offset into the GUID array
            let offset = idx / 16;

            // Bounds check
            if offset == 0 || offset >= data.len() {
                None
            } else {
                Some(&data[offset])
            }
        } else {
            None
        }
    }
}

impl<'a> ::std::ops::Index<usize> for GuidHeap<'a> {
    type Output = Guid;

    fn index(&self, idx: usize) -> &Guid {
        self.get(idx).expect("Expected a valid Heap reference")
    }
}
