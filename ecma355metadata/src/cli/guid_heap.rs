use error::Error;

use Guid;

pub struct GuidHeap<'a> {
    data: Option<&'a [Guid]>,
}

impl<'a> GuidHeap<'a> {
    pub fn new(data: &'a [Guid]) -> GuidHeap<'a> {
        GuidHeap { data: Some(data) }
    }

    pub fn empty() -> GuidHeap<'a> {
        GuidHeap { data: None }
    }

    pub fn get(&self, idx: usize) -> Result<&Guid, Error> {
        if let Some(data) = self.data {
            // Determine the offset into the GUID array
            let offset = idx / 16;

            // Bounds check
            if offset >= data.len() {
                Err(Error::InvalidHeapReference)
            } else {
                Ok(&data[offset])
            }
        } else {
            Err(Error::InvalidHeapReference)
        }
    }
}

impl<'a> ::std::ops::Index<usize> for GuidHeap<'a> {
    type Output = Guid;

    fn index(&self, idx: usize) -> &Guid {
        self.get(idx).expect("Expected a valid Heap reference")
    }
}
