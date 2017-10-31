use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use error::Error;

#[derive(Debug, Eq, PartialEq)]
pub enum DirectoryType {
    ExportTable,
    ImportTable,
    ResourceTable,
    ExceptionTable,
    CertificateTable,
    BaseRelocationTable,
    DebugData,
    CopyrightData,
    GlobalPtrData,
    TlsTable,
    LoadConfigTable,
    BoundImport,
    ImportAddressTable,
    DelayImportDescriptor,
    CliHeader,
    Reserved,
}

impl ::std::fmt::Display for DirectoryType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        let s = match *self {
            DirectoryType::ExportTable => "Export Table",
            DirectoryType::ImportTable => "Import Table",
            DirectoryType::ResourceTable => "Resource Table",
            DirectoryType::ExceptionTable => "Exception Table",
            DirectoryType::CertificateTable => "Certificate Table",
            DirectoryType::BaseRelocationTable => "Base Relocation Table",
            DirectoryType::DebugData => "Debug Data",
            DirectoryType::CopyrightData => "Copyright Data",
            DirectoryType::GlobalPtrData => "Global Pointer Data",
            DirectoryType::TlsTable => "Thread Local Storage",
            DirectoryType::LoadConfigTable => "Loader Configuration Table",
            DirectoryType::BoundImport => "Bound Import Table",
            DirectoryType::ImportAddressTable => "Import Address Table",
            DirectoryType::DelayImportDescriptor => "Delay Import Descriptor",
            DirectoryType::CliHeader => "CLI Header",
            DirectoryType::Reserved => "Reserved",
        };

        f.write_str(s)
    }
}

#[derive(Debug)]
pub struct DirectoryEntry {
    pub directory_type: DirectoryType,
    pub rva: u32,
    pub size: u32,
}

impl DirectoryEntry {
    pub fn new(directory_type: DirectoryType, rva: u32, size: u32) -> DirectoryEntry {
        DirectoryEntry {
            directory_type: directory_type,
            rva: rva,
            size: size,
        }
    }

    pub fn read<A: Read>(directory_type: DirectoryType, buf: &mut A) -> Result<DirectoryEntry, Error> {
        Ok(DirectoryEntry::new(
            directory_type,
            buf.read_u32::<LittleEndian>()?,
            buf.read_u32::<LittleEndian>()?,
        ))
    }
}

impl ::std::fmt::Display for DirectoryEntry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(
            f,
            "0x{:08X} (Size: 0x{:08X}): {}",
            self.rva,
            self.size,
            self.directory_type
        )
    }
}
