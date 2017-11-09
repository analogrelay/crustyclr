extern crate ecma355metadata;

use std::env;
use std::fs::File;
use std::io::Cursor;

use ecma355metadata::PeImage;
use ecma355metadata::cli::CliHeader;
use ecma355metadata::pe::DirectoryType;

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
        println!("Usage: dump_asm <file>");
    } else {
        let mut file = File::open(&args[1]).unwrap();
        let pe = PeImage::read(&mut file).unwrap();
        let cli_header = load_cli_header(&pe);
        // let assembly = MetadataReader::new(
        //     pe.load(cli_header.metadata.rva, cli_header.metadata.size as usize)
        //         .unwrap(),
        // ).unwrap();

        println!("CLI Header");
        println!("  Size: {}", cli_header.header_size);
        println!(
            "  Runtime Version: {}.{}",
            cli_header.major_runtime_version,
            cli_header.minor_runtime_version
        );
        println!("  Metadata: {}", cli_header.metadata);
        println!("  Flags: {}", cli_header.flags);
        println!("  Entrypoint Token: 0x{:08X}", cli_header.entry_point_token);
        println!("  Resources: {}", cli_header.resources);
        println!("  Strong Name: {}", cli_header.strong_name);
        println!("  Code Manager Table: {}", cli_header.code_manager_table);
        println!("  VTable Fixups: {}", cli_header.vtable_fixups);
        println!(
            "  Export Address Table Jumps: {}",
            cli_header.export_address_table_jumps
        );
        println!(
            "  Managed/Native Header: {}",
            cli_header.managed_native_header
        );
        println!();

        // println!("Metadata Header:");
        // println!(
        //     "  Version: {}.{} ({})",
        //     assembly.metadata_header().major_version,
        //     assembly.metadata_header().minor_version,
        //     assembly.metadata_header().version
        // );
        // println!("  Flags: 0x{:04X}", assembly.metadata_header().flags);
        // println!("  Streams: {}", assembly.metadata_header().streams);
        // println!();

        // println!("Streams:");
        // for stream_header in assembly.stream_headers() {
        //     println!(
        //         "  [0x{:08X} - 0x{:08X}] {}",
        //         stream_header.offset,
        //         stream_header.offset + stream_header.size,
        //         stream_header.name
        //     );
        // }
        // println!();

        // println!("Tables:");
        // println!(
        //     "  Version: {}.{}",
        //     assembly.table_list().major_version,
        //     assembly.table_list().minor_version
        // );
        // println!("  HeapSizes: {}", assembly.table_list().heap_sizes);
        // for table in assembly.table_list().tables() {
        //     println!(
        //         "  {}: {} rows, {}",
        //         table.table,
        //         table.rows,
        //         if table.sorted { "Sorted" } else { "Unsorted" }
        //     );
        // }
    }
}
