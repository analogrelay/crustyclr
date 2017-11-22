extern crate crustyclr;

use std::env;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: crustyrun <assembly>");
    } else {
        let app_path = Path::new(&args[1]).canonicalize().unwrap();
        let assembly = app_path.file_name().unwrap();
        let base_dir = app_path.parent().unwrap();
        println!("AppPath={:?}", app_path);
        println!("Assembly={:?}", assembly);
        println!("BaseDir={:?}", base_dir);
    }
}
