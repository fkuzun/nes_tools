use std::process::Command;
use serde::de::Error;
use serde_json::to_string;
pub mod coordinator;
pub mod worker;

pub trait Launch {
    fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn launch_with_pipe(&mut self) -> Result<(), Box<dyn std::error::Error>>;

    fn kill(&mut self) -> Result<(), Box<dyn std::error::Error>>;
}

pub trait CommandLineArgument {
    fn as_command_line_argument(&self) -> Option<String>;
}

