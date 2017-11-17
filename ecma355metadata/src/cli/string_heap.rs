use std::ffi::CStr;
use std::mem;

pub struct StringHeap<'a> {
    data: Option<&'a [u8]>,
}

impl<'a> StringHeap<'a> {
    const EMPTY: StringHeap<'static> = StringHeap { data: None }

    pub fn new(data: &'a [u8]) -> StringHeap<'a> {
        StringHeap { data: Some(data) }
    }

    pub fn get(&self, idx: usize) -> Option<&[u8]> {
        if let Some(data) = self.data {
            // Bounds check
            if idx == 0 || idx >= data.len() {
                None
            } else {
                unsafe {
                    // Find the start point and convert it to an unsafe pointer
                    let ptr = mem::transmute(&data[idx]);

                    // Load as a CStr
                    let cstr = CStr::from_ptr(ptr);

                    // Convert to an &str and return it
                    Some(cstr.to_bytes())
                }
            }
        } else {
            None
        }
    }
}

impl<'a> ::std::ops::Index<usize> for StringHeap<'a> {
    type Output = [u8];

    fn index(&self, idx: usize) -> &[u8] {
        self.get(idx).expect("Expected a valid Heap reference")
    }
}
