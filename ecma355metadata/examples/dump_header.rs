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

        println!("Header: {:?}", pe.header());
    }
}
