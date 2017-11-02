use format::DirectoryType;

pub trait Directory {
    const TYPE: DirectoryType;
}
