use error::Error;

pub struct Runtime;

impl Runtime {
    pub fn new() -> Runtime {
        Runtime
    }

    pub fn execute(&self, path: &str) -> Result<(), Error> {
        unimplemented!()
    }
}
