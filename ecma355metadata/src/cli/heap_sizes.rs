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