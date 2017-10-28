extern crate ecma355metadata;

use std::env;
use std::fs::File;

use ecma355metadata::PortableExecutable;

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dump_header <file>");
    } else {
        let mut file = File::open(&args[1]).unwrap();
        let pe = PortableExecutable::read(&mut file).unwrap();

        let coff_header = pe.coff_header();
        println!("COFF Header:");
        println!("  Machine: 0x{:X}", coff_header.machine);
        println!("  Number of Sections: {}", coff_header.number_of_sections);
        println!("  Timestamp: {}", coff_header.timestamp);
        println!(
            "  Symbol Table Offset: 0x{:X}",
            coff_header.symbol_table_addr
        );
        println!("  Symbol Count: {}", coff_header.symbol_count);
        println!(
            "  Optional Header Size: {}",
            coff_header.optional_header_size
        );
        println!("  Characteristics: 0x{:X}", coff_header.characteristics);

        println!();
        if let Some(pe_header) = pe.pe_header() {
            println!("PE Header:");
            println!("  Magic: 0x{:X}", pe_header.magic);
            println!("  Major Linker Version: {}", pe_header.major_linker_version);
            println!("  Minor Linker Version: {}", pe_header.minor_linker_version);
            println!("  Code Size: {}", pe_header.code_size);
            println!(
                "  Initialized Data Size: {}",
                pe_header.initialized_data_size
            );
            println!(
                "  Uninitialized Data Size: {}",
                pe_header.uninitialized_data_size
            );
            println!("  Entrypoint RVA: 0x{:X}", pe_header.entry_point_rva);
            println!("  Base of Code: 0x{:X}", pe_header.code_base);
            println!("  Base of Data: 0x{:X}", pe_header.data_base);
        }
    }
}
