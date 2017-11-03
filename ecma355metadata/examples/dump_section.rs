extern crate ecma355metadata;

use std::env;
use std::fs::File;
use std::io::Read;
use std::ascii::AsciiExt;

use ecma355metadata::pe::PeImage;

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
        let mut pe = PeImage::read(&mut file).unwrap();

        // Get the section
        if let Some(section) = pe.get_section(section_name) {
            println!("Section: {}", section.header().name);
            println!();
            let mut reader = section.create_reader();
            let mut buf = [0u8; STRIDE];
            let mut line_number = 0;
            loop {
                if line_number % 20 == 0 {
                    println!();
                    print_headers();
                }

                let read = reader.read(&mut buf).unwrap();
                if read == 0 {
                    return;
                }

                let offset = line_number * STRIDE;
                print!("0x{:04X} | ", offset);
                for index in 0..STRIDE {
                    print!("{:02X} ", buf[index]);
                }
                print!(" | ");
                for index in 0..STRIDE {
                    let val = buf[index];
                    let ch = if is_printable(val) { val.into() } else { '.' };
                    print!("{}", ch);
                }
                println!();

                line_number += 1;
            }
        }
    }
}
