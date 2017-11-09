use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{GuidRef, HeapRef, HeapSizes, StringRef};
use error::Error;

pub struct Module {
    pub generation: u16,
    pub name: StringRef,
    pub mvid: GuidRef,
    pub enc_id: GuidRef,
    pub enc_base_id: GuidRef,
}

impl Module {
    pub fn read<R: Read>(reader: &mut R, heap_sizes: HeapSizes) -> Result<Module, Error> {
        Ok(Module {
            generation: reader.read_u16::<LittleEndian>()?,
            name: StringRef::read(reader, heap_sizes)?,
            mvid: GuidRef::read(reader, heap_sizes)?,
            enc_id: GuidRef::read(reader, heap_sizes)?,
            enc_base_id: GuidRef::read(reader, heap_sizes)?,
        })
    }
}

pub struct ModuleTable<R: Read> {
    reader: R,
    heap_sizes: HeapSizes,
}

impl<R: Read> ModuleTable<R> {
    pub fn new(reader: R, heap_sizes: HeapSizes) -> ModuleTable<R> {
        ModuleTable {
            reader: reader,
            heap_sizes: heap_sizes,
        }
    }
}

impl<R: Read> Iterator for ModuleTable<R> {
    type Item = Result<Module, Error>;

    fn next(&mut self) -> Option<Result<Module, Error>> {
        match Module::read(&mut self.reader, self.heap_sizes) {
            Ok(o) => Some(Ok(o)),
            Err(Error::IoError(ref io)) if io.kind() == ::std::io::ErrorKind::UnexpectedEof => None,
            Err(e) => Some(Err(e)),
        }
    }
}
