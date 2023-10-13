use std::process::Command;
use serde::de::Error;
use serde_json::to_string;
pub mod coordinator;

pub trait Launch {
    fn launch(&mut self) -> Result<(), Box<dyn Error>>;
}

