extern crate ecma355metadata;

use std::env;
use std::str;
use std::fs::File;
use std::io::Cursor;

use ecma355metadata::{MetadataReader, PeImage};
use ecma355metadata::cli::CliHeader;
use ecma355metadata::cli::tables::{Module, TypeRef};
use ecma355metadata::pe::DirectoryType;
use ecma355metadata::Guid;

fn load_cli_header(pe: &PeImage) -> CliHeader {
    let (rva, size) = pe.get_directory(DirectoryType::CliHeader)
        .map(|d| (d.range.rva, d.range.size))
        .unwrap();

    let mut reader = Cursor::new(pe.load(rva, size as usize).unwrap());

    CliHeader::read(&mut reader).unwrap()
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_table <file>");
    } else {
        let mut file = File::open(&args[1]).unwrap();
        let pe = PeImage::read(&mut file).unwrap();
        let cli_header = load_cli_header(&pe);
        let assembly = MetadataReader::new(
            pe.load(cli_header.metadata.rva, cli_header.metadata.size as usize)
                .unwrap(),
        ).unwrap();

        let string_heap = assembly.string_heap();
        let guid_heap = assembly.guid_heap();

        let module_table = assembly.table::<Module>();
        println!("Module Table: {} rows", module_table.len());
        for row in module_table.iter() {
            let row = row.unwrap();
            println!("  Generation: {}", row.generation);
            println!("  Name: {}", str::from_utf8(row.name.get(string_heap).unwrap()).unwrap());
            println!("  MVID: {}", row.mvid.get(guid_heap).unwrap_or(&Guid::EMPTY));
        }
        println!();

        let type_ref_table = assembly.table::<TypeRef>();
        println!("TypeRef Table: {} rows", type_ref_table.len());
        for row in type_ref_table.iter() {
            let row = row.unwrap();
            let name = row.name.get(string_heap).unwrap_or(b"<null>");
            let namespace = row.namespace.get(string_heap);

            if let Some(ns) = namespace {
                println!(" * {}.{} (Scope: 0x{:04X})", str::from_utf8(ns).unwrap(), str::from_utf8(name).unwrap(), row.resolution_scope);
            }
            else {
                println!(" * {} (Scope: 0x{:04X})", str::from_utf8(name).unwrap(), row.resolution_scope);
            }
        }
        println!()
    }
}
