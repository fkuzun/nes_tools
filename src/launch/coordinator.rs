use std::process::{Child, Command};
use serde::de::Error;
use crate::launch::Launch;

pub struct Coordinator {
    command: Command,
    process: Child,
}

impl Launch for Coordinator {
    fn launch(&mut self) -> Result<(), Box<dyn Error>> {
        self.process = self.command.spawn()?;
        Ok(())
    }
}

