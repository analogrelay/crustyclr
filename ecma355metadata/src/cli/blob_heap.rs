pub struct BlobHeap<'a> {
    data: Option<&'a [u8]>,
}

impl<'a> BlobHeap<'a> {
    pub const EMPTY: BlobHeap<'static> = BlobHeap { data: None };

    pub fn new(data: &'a [u8]) -> BlobHeap<'a> {
        BlobHeap { data: Some(data) }
    }

    pub fn get(&self, idx: usize) -> Option<&[u8]> {
        if let Some(data) = self.data {
            // Bounds check
            if idx == 0 || idx >= data.len() {
                None
            } else {
                // Read the header
                if data[idx] & 0x80 == 0 {
                    // 1-byte length
                    let start = idx + 1;
                    let len = (data[idx] as usize) & 0x7F;
                    Some(&data[start..(start + len)])
                } else if data[idx] & 0xC0 == 0 {
                    // 2-byte length
                    let start = idx + 2;
                    let len = ((data[idx] as usize & 0x3F) << 8) + data[idx + 1] as usize;
                    Some(&data[start..(start + len)])
                } else {
                    // 4-byte length
                    let start = idx + 4;
                    let len = ((data[idx] as usize) & 0x1F << 24) + ((data[idx + 1] as usize) << 16)
                        + ((data[idx + 2] as usize) << 8)
                        + data[idx + 3] as usize;
                    Some(&data[start..(start + len)])
                }
            }
        } else {
            None
        }
    }
}

impl<'a> ::std::ops::Index<usize> for BlobHeap<'a> {
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8] {
        self.get(idx).expect("Expected a valid Heap reference")
    }
}
