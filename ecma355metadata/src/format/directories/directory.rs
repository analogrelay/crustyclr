use std::io::Read;

use format::DirectoryType;

use error::Error;

pub trait Directory {
    const TYPE: DirectoryType;

    fn read<R: Read>(r: &mut R) -> Result<Self, Error>;
}
