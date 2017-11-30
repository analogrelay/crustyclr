use std::fs::File;
use std::path::{Path, PathBuf};

use slog;
use memmap;

use error::Error;
use assembly::Assembly;
use loader;

pub struct AppContext {
    base_directory: PathBuf,
    logger: slog::Logger,
}

impl AppContext {
    pub fn new<P: Into<PathBuf>>(base_directory: P, logger: slog::Logger) -> AppContext {
        AppContext {
            base_directory: base_directory.into(),
            logger: logger,
        }
    }

    pub fn load(&mut self, assembly_name: &str) -> Result<Assembly, Error> {
        // Resolve the path
        let logger = self.logger
            .new(o!("assembly_name" => assembly_name.to_owned()));
        let assembly_path = resolve_assembly(
            &self.base_directory,
            assembly_name,
            &logger,
        )?;

        info!(logger, "loading {} from {}", assembly_name, assembly_path.display());

        // Load the file into memory
        let file = File::open(assembly_path)?;
        let mmap = unsafe {
            memmap::MmapOptions::new()
                .map(&file)?
        };

        let asm = Assembly::load(mmap, &logger)?;
        unimplemented!()
    }
}

const ASSEMBLY_EXTENSIONS: [&'static str; 2] = ["exe", "dll"];
fn resolve_assembly(
    base_directory: &Path,
    assembly_name: &str,
    logger: &slog::Logger,
) -> Result<PathBuf, Error> {
    let result = ASSEMBLY_EXTENSIONS
        .iter()
        .map(|ext| {
            let p = PathBuf::from(assembly_name).with_extension(ext);
            base_directory.join(p)
        })
        .find(|p| {
            debug!(logger, "trying path: {}", p.display(); "candidate_path" => p.to_path_buf().display());
            p.exists()
        });
    result.ok_or(Error::AssemblyNotFound(assembly_name.into()))
}
