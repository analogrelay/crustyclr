extern crate crustyclr;

#[macro_use]
extern crate slog;
extern crate sloggers;

use std::env;
use std::path::Path;

use sloggers::{Build, set_stdlog_logger};
use sloggers::terminal::{Destination, TerminalLoggerBuilder};
use sloggers::types::{Format, Severity};

use crustyclr::RuntimeBuilder;

fn main() {
    // Need to use a wrapping function to force the destructors
    // inside do_main to run. process::exit just aborts the process
    // without cleaining things up, and that means slog doesn't write
    // logs out (because it uses an async buffer).

    let exit_code = do_main();
    ::std::process::exit(exit_code);
}

fn do_main() -> i32 {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: crustyrun <assembly>");
        1
    } else {
        let app_path = Path::new(&args[1]).canonicalize().unwrap();
        let assembly = app_path.file_stem().unwrap().to_str().unwrap();
        let base_dir = app_path.parent().unwrap();

        // Set up logging
        let mut builder = TerminalLoggerBuilder::new();
        builder.format(Format::Full);
        builder.level(Severity::Debug);
        builder.destination(Destination::Stderr);

        let logger = builder.build().unwrap();

        set_stdlog_logger(logger.new(o!("stdlog" => true))).unwrap();

        // Create a runtime
        let mut rt = RuntimeBuilder::new()
            .base_directory(base_dir)
            .logger(logger)
            .build();

        // Execute the assembly
        rt.execute(assembly).unwrap()
    }
}
