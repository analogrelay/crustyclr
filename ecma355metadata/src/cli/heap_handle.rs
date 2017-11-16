use std::io::Read;
use std::mem::size_of;

use byteorder::{LittleEndian, ReadBytesExt};

use cli::{HeapSizes, MetadataSizes, SMALL_INDEX_SIZE, LARGE_INDEX_SIZE};
use error::Error;

pub struct StringHandle(usize);

impl StringHandle {
	const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

	fn new(index: usize) -> StringHandle {
		StringHandle(index)
	}

	fn read<R: Read>(reader: &mut R, large: bool) -> Result<StringHandle, Error> {
		StringHandle(read_heap_handle(reader, large)?)
	}

	fn index(&self) -> usize {
		self.0
	}
}

pub struct StringHandleReader(bool);

impl StringHandleReader {
	pub fn new(large: bool) -> StringHandleReader {
		StringHandleReader(large)
	}

	pub fn size(&self) -> usize) {
		if self.0 {
			LARGE_INDEX_SIZE   
		} else {
			SMALL_INDEX_SIZE
		}
	}

	pub fn read<R: Read>(reader: &mut R) -> Result<StringHandle, Error> {
		StringHandle::read(reader, self.0)
	}
}

pub struct GuidHandle(usize);

impl GuidHandle {
	const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

	fn new(index: usize) -> GuidHandle {
		GuidHandle(index)
	}

	fn read<R: Read>(reader: &mut R, large: bool) -> Result<GuidHandle, Error> {
		GuidHandle(read_heap_handle(reader, large)?)
	}

	fn is_large(sizes: &MetadataSizes) -> bool {
		sizes.heap_sizes().contains(GuidHandle::SIZE_FLAG)
	}

	fn index(&self) -> usize {
		self.0
	}
}

pub struct GuidHandleReader(bool);

impl GuidHandleReader {
	pub fn new(large: bool) -> GuidHandleReader {
		GuidHandleReader(large)
	}

	pub fn size(&self) -> usize) {
		if self.0 {
			LARGE_INDEX_SIZE   
		} else {
			SMALL_INDEX_SIZE
		}
	}

	pub fn read<R: Read>(reader: &mut R) -> Result<GuidHandle, Error> {
		GuidHandle::read(reader, self.0)
	}
}

pub struct BlobHandle(usize);

impl BlobHandle {
	const SIZE_FLAG: HeapSizes = HeapSizes::LARGE_STRINGS;

	fn new(index: usize) -> BlobHandle {
		BlobHandle(index)
	}

	fn read<R: Read>(reader: &mut R, large: bool) -> Result<BlobHandle, Error> {
		BlobHandle(read_heap_handle(reader, large)?)
	}

	fn is_large(sizes: &MetadataSizes) -> bool {
		sizes.heap_sizes().contains(BlobHandle::SIZE_FLAG)
	}

	fn index(&self) -> usize {
		self.0
	}
}

pub struct BlobHandleReader(bool);

impl BlobHandleReader {
	pub fn new(large: bool) -> BlobHandleReader {
		BlobHandleReader(large)
	}

	pub fn size(&self) -> usize) {
		if self.0 {
			LARGE_INDEX_SIZE   
		} else {
			SMALL_INDEX_SIZE
		}
	}

	pub fn read<R: Read>(reader: &mut R) -> Result<BlobHandle, Error> {
		BlobHandle::read(reader, self.0)
	}
}

fn read_heap_handle<R>(reader: &mut R, large: bool) -> Result<usize, Error> {
	if large {
		reader.read_u32::<LittleEndian>()? as usize
	} else {
		reader.read_u16::<LittleEndian>()? as usize
	}
}
