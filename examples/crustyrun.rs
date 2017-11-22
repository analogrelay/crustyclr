extern crate crustyclr;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: crustyrun <assembly>");
    } else {
        let assembly = &args[1];
        let runtime = crustyclr::Runtime::new();
        runtime.execute(assembly).unwrap();
    }
}
