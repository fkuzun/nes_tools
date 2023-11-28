use std::process::{Child, Command};
use serde::de::Error;
use crate::launch::Launch;

pub struct Coordinator {
    command: Command,
    process: Option<Child>,
}

impl Coordinator {
    pub fn new(path: &str) ->Self {
        Self {
            command: Command::new(path),
            process: None
        }
    }
}

//todo: this should hide if the coordinator is running locally or not
impl Launch for Coordinator {
    fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.process = Some(self.command.spawn()?);
        Ok(())
    }

    fn launch_with_pipe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }

    fn kill(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.process.take().ok_or("No process exists")?.kill().or(Err("Could not kill process".into()))
    }
}

