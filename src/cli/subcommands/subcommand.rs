use std::error::Error;

pub(crate) trait Subcommand {
    fn execute(&self) -> Result<(), Box<dyn Error>>;
}
