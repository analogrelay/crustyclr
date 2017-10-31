use std::io::Read;

use format::DirectoryType;

pub trait DirectoryReader<'a> {
    const TYPE: DirectoryType;

    fn read(r: &'a mut Read) -> Self;
}

pub struct CliHeaderReader<'a>(&'a mut Read);

impl<'a> DirectoryReader<'a> for CliHeaderReader<'a> {
    const TYPE: DirectoryType = DirectoryType::CliHeader;

    fn read(r: &'a mut Read) -> CliHeaderReader<'a> {
        CliHeaderReader(r)
    }
}
