extern crate ecma355metadata;

use std::env;
use std::fs::File;
use std::ascii::AsciiExt;

use ecma355metadata::PeReader;

const STRIDE: usize = 32; 

fn is_printable(val: u8) -> bool {
    val >= b' ' && val <= b'~'
}

fn print_headers() {
    print!("         ");
    for index in 0..STRIDE {
        print!("{:02X} ", index);
    }
    println!();
    print!("         ");
    for index in 0..STRIDE {
        print!("-- ");
    }
    println!();
}

pub fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: dump_header <file> <section>");
    } else {
        let filename = &args[1];
        let section_name = &args[2];
        let mut file = File::open(filename).unwrap();
        let mut pe = PeReader::read(&mut file).unwrap();

        let section = pe.get_section(section_name).unwrap();

        println!("Section: {}", section_name);
        println!();
        let lines = section.len() / STRIDE;
        for line_number in 0..lines {
            if line_number % 20 == 0 {
                println!();
                print_headers();
            }

            let offset = line_number * STRIDE;
            print!("0x{:04X} | ", offset);
            for index in 0..STRIDE {
                print!("{:02X} ", section[offset + index]);
            }
            print!(" | ");
            for index in 0..STRIDE {
                let val = section[offset + index];
                let ch = if is_printable(val) {
                   val.into() 
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();
        }
    }
}