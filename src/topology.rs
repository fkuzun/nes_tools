use std::io::Read;
use std::net;
use std::ops::Add;
use serde::{Serialize, Deserialize};
use crate::launch::CommandLineArgument;

pub type TopologyNodeId = u64;

#[derive(Serialize, Deserialize, Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub struct Edge {
    source: TopologyNodeId,
    target: TopologyNodeId
}

impl Edge {
    pub fn get_source(&self) -> TopologyNodeId {
        self.source
    }

    pub fn get_target(&self) -> TopologyNodeId {
        self.target
    }

}

#[derive(Serialize, Deserialize, Debug, PartialEq, Copy, Clone)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeSpatialType {
    NO_LOCATION,
    FIXED_LOCATION,
    MOBILE_NODE,
    INVALID
}

impl CommandLineArgument for NodeSpatialType {
    fn as_command_line_argument(&self) -> Option<String> {
        let prefix = String::from("--nodeSpatialType=");
        match self {
            Self::NO_LOCATION => None,
            Self::MOBILE_NODE => Some(prefix.add("MOBILE_NODE")),
            Self::FIXED_LOCATION => Some(prefix.add("FIXED_LOCATION")),
            Self::INVALID => panic!()
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    available_resources: u16,
    id: TopologyNodeId,
    ip_address: net::IpAddr,
    location: Option<Location>,
    nodeType: NodeSpatialType,
}

pub trait GeoNode {
    fn get_location(&self) -> Option<&Location>;
    fn get_id(&self) -> TopologyNodeId;
}
impl GeoNode for Node {
    fn get_location(&self) -> Option<&Location> {
        self.location.as_ref()
    }
    fn get_id(&self) -> TopologyNodeId {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MobileNode {
    id: TopologyNodeId,
    location: Location,
}

impl GeoNode for MobileNode {
    fn get_location(&self) -> Option<&Location> {
        Some(&self.location)
    }
    fn get_id(&self) -> TopologyNodeId {
        self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topology {
    edges: Vec<Edge>,
    nodes: Vec<Node>
}

impl Topology {
    pub fn empty() -> Self {
        Self {
            edges: vec![],
            nodes: vec![]
        }
    }
    pub fn get_nodes(&self) -> &Vec<Node> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }
}

// pub fn get_topology() -> Result<Topology, Box<dyn std::error::Error>> {
//     //todo allow setting the url
//     let mut res = reqwest::blocking::get("http://127.0.0.1:8081/v1/nes/topology")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);
//     Ok(serde_json::from_str(&body)?)
// }
//
// pub fn get_all_mobile() -> Result<AllMobile, Box<dyn std::error::Error>> {
//     let mut res = reqwest::blocking::get("http://127.0.0.1:8081/v1/nes/location/allMobile")?;
//     let mut body = String::new();
//     res.read_to_string(&mut body)?;
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);
//     Ok(serde_json::from_str(&body)?)
// }

#[derive(Serialize, Deserialize)]
pub struct AllMobile {
    edges: Vec<Edge>,
    nodes: Vec<MobileNode>,
}

impl AllMobile {
    pub fn empty() -> Self {
        Self {
            edges: vec![],
            nodes: vec![]
        }
    }
    pub fn get_nodes(&self) -> &Vec<MobileNode> {
        &self.nodes
    }

    pub fn get_edges(&self) -> &Vec<Edge> {
        &self.edges
    }
}

#[derive(Serialize, Deserialize)]
pub struct AddEdgeRequest {
    #[serde(rename = "parentId")]
    pub parent_id: u64,
    #[serde(rename = "childId")]
    pub child_id: u64,
}

#[derive(Serialize, Deserialize)]
pub struct AddEdgeReply {
    pub success: bool
}


#[derive(Serialize, Deserialize)]
pub struct ExecuteQueryRequest {
    #[serde(rename = "userQuery")]
    pub user_query: String,
    pub placement: PlacementStrategyType,
}

#[derive(Serialize, Deserialize)]
pub enum PlacementStrategyType {
    BottomUp,
    TopDown,
}