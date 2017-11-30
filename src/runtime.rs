use std::env;
use std::path::{Path, PathBuf};

use slog;

use error::Error;
use app_context::AppContext;

pub struct RuntimeBuilder {
    base_directory: Option<PathBuf>,
    logger: Option<slog::Logger>,
}

impl RuntimeBuilder {
    /// Creates a runtime builder.
    pub fn new() -> RuntimeBuilder {
        RuntimeBuilder {
            base_directory: None,
            logger: None,
        }
    }

    /// Consumes the builder and creates an Runtime from the result.
    pub fn build(self) -> Runtime {
        Runtime::new(
            self.base_directory.unwrap_or_else(|| {
                env::current_dir().expect("Failed to get the current directory")
            }),
            self.logger
                .unwrap_or_else(|| slog::Logger::root(slog::Discard, o!())))
    }

    /// Sets the base directory for the Runtime and returns the builder (for method chaining)
    pub fn base_directory(mut self, base_directory: &Path) -> RuntimeBuilder {
        self.base_directory = Some(base_directory.into());
        self
    }

    pub fn logger(mut self, logger: slog::Logger) -> RuntimeBuilder {
        self.logger = Some(logger);
        self
    }
}

pub struct Runtime {
    app_context: AppContext,
    logger: slog::Logger,
}

impl Runtime {
    fn new(base_directory: PathBuf, logger: slog::Logger) -> Runtime {
        let base_dir_str = base_directory.clone().into_os_string().into_string().expect("Unable to convert path to string!");
        Runtime {
            app_context: AppContext::new(
                base_directory,
                logger.new(o!("base_directory" => base_dir_str)),
            ),
            logger: logger,
        }
    }

    pub fn execute(&mut self, assembly_name: &str) -> Result<i32, Error> {
        debug!(self.logger, "executing assembly"; "assembly" => assembly_name);

        // Load the assembly
        let assembly = self.app_context.load(assembly_name)?;

        Ok(1)
    }
}
