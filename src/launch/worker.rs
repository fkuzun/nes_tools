use std::ops::Add;
use crate::launch::{CommandLineArgument, Launch};
use std::process::{Child, Command};
use crate::config;

pub struct Worker {
    command: Command,
    process: Option<Child>,
}

impl Worker {
    pub fn from_config_path(path: &str, config_path: &str) -> Self {
        let mut command = Command::new(path);
        command.arg(String::from("--configPath=").add(config_path));
        Self {
            command,
            process: None,
        }
    }
    pub fn new(path: &str, config: Option<config::worker_config::WorkerConfig>) -> Self {
        let mut command = Command::new(path);
        if let Some(worker_config) = config {
            if let Some(spatial_type_string) = worker_config.node_spatial_type.as_command_line_argument() {
                command.arg(spatial_type_string);
            }
            let mobility_config = &worker_config.worker_mobility_config;
            if let Some(location_provider_type_string) = mobility_config.location_provider_type.as_command_line_argument() {
                command.arg(location_provider_type_string);
            }
            if let Some(location_provider_config_string) = mobility_config.location_provider_config.as_command_line_argument() {
                command.arg(location_provider_config_string);
            }
        }
        Self {
            command,
            process: None,
        }
    }
}

//todo: check if this code duplication with coordinator can be omitted even further (check chapter on oop patterns)
// impl Launch for Worker {
//
//     fn launch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         *self.get_process() = Some(self.get_command().spawn()?);
//         Ok(())
//     }
//
//     fn launch_with_pipe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         todo!()
//     }
//
//     fn kill(&mut self) -> Result<(), Box<dyn std::error::Error>> {
//         self.get_process().take().ok_or("No process exists")?.kill().or(Err("Could not kill process".into()))
//     }
// }
