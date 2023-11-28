use std::ops::Add;
use crate::launch::CommandLineArgument;
use crate::topology::NodeSpatialType;

pub enum LocationProviderType {
    NONE,
    CSV
}


impl CommandLineArgument for LocationProviderType {
    fn as_command_line_argument(&self) -> Option<String> {
        match self {
            Self::NONE => None,
            Self::CSV => Some(String::from("--locationProviderType=CSV"))
        }
    }
}
pub struct LocationProviderConfig {
    path: String
}

impl LocationProviderConfig {
    pub fn new(path: &str) -> Self {
        Self {
            path: path.to_owned()
        }
    }

}

impl CommandLineArgument for LocationProviderConfig {
    fn as_command_line_argument(&self) -> Option<String> {
            Some(String::from("--locationProviderConfig=").add(&self.path))
    }
}
pub struct WorkerConfig {
    pub worker_mobility_config: WorkerMobilityConfig,
    pub node_spatial_type: NodeSpatialType
}

pub struct WorkerMobilityConfig {
    pub location_provider_type: LocationProviderType,
    pub location_provider_config: LocationProviderConfig
}

pub struct ConfigPath {
    path: String
}
impl CommandLineArgument for ConfigPath {
    fn as_command_line_argument(&self) -> Option<String> {
        Some(String::from("--configPath=").add(&self.path))
    }
}
