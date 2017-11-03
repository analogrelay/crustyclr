extern crate ecma355metadata;

use std::env;
use std::fs::File;

use ecma355metadata::MetadataReader;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_asm <file>");
    } else {
        let mut file = File::open(&args[1]).unwrap();
        let assembly = MetadataReader::new(&mut file).unwrap();

        println!("CLI Header");
        println!("  Size: {}", assembly.cli_header().header_size);
        println!(
            "  Runtime Version: {}.{}",
            assembly.cli_header().major_runtime_version,
            assembly.cli_header().minor_runtime_version
        );
        println!("  Metadata: {}", assembly.cli_header().metadata);
        println!("  Flags: {}", assembly.cli_header().flags);
        println!("  Entrypoint Token: 0x{:08X}", assembly.cli_header().entry_point_token);
        println!("  Resources: {}", assembly.cli_header().resources);
        println!("  Strong Name: {}", assembly.cli_header().strong_name);
        println!("  Code Manager Table: {}", assembly.cli_header().code_manager_table);
        println!("  VTable Fixups: {}", assembly.cli_header().vtable_fixups);
        println!(
            "  Export Address Table Jumps: {}",
            assembly.cli_header().export_address_table_jumps
        );
        println!(
            "  Managed/Native Header: {}",
            assembly.cli_header().managed_native_header
        );
        println!();

        println!("Metadata Header");
        println!("  Version: {}.{} ({})", assembly.metadata_header().major_version, assembly.metadata_header().minor_version, assembly.metadata_header().version);
        println!("  Flags: 0x{:04X}", assembly.metadata_header().flags);
        println!("  Streams: {}", assembly.metadata_header().streams);
    }
}
