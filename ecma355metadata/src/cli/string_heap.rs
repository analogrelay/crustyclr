use std::ffi::CStr;
use std::mem;

use error::Error;

pub struct StringHeap<'a> {
    data: Option<&'a [u8]>,
}

impl<'a> StringHeap<'a> {
    pub fn new(data: &'a [u8]) -> StringHeap<'a> {
        StringHeap { data: Some(data) }
    }

    pub fn empty() -> StringHeap<'a> {
        StringHeap { data: None }
    }

    pub fn get(&self, idx: usize) -> Result<&str, Error> {
        if let Some(data) = self.data {
            // Bounds check
            if idx >= data.len() {
                Err(Error::InvalidHeapReference)
            } else {
                unsafe {
                    // Find the start point and convert it to an unsafe pointer
                    let ptr = mem::transmute(&data[idx]);

                    // Load as a CStr
                    let cstr = CStr::from_ptr(ptr);

                    // Convert to an &str and return it
                    Ok(cstr.to_str()?)
                }
            }
        } else {
            Err(Error::InvalidHeapReference)
        }
    }
}

impl<'a> ::std::ops::Index<usize> for StringHeap<'a> {
    type Output = str;

    fn index(&self, idx: usize) -> &str {
        self.get(idx).expect("Expected a valid Heap reference")
    }
}
