use std::io::Read;
use std::net;
use serde::{Serialize, Deserialize};

pub type TopologyNodeId = usize;


#[derive(Serialize, Deserialize, Debug)]
pub struct Edge {
    source: TopologyNodeId,
    target: TopologyNodeId
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    latitude: f64,
    longitude: f64
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NodeSpatialType {
    NO_LOCATION,
    FIXED_LOCATION,
    MOBILE_NODE,
    INVALID
}
#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    available_resources: u16,
    id: TopologyNodeId,
    ip_address: net::IpAddr,
    location: Option<Location>,
    nodeType: NodeSpatialType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Topology {
    edges: Vec<Edge>,
    nodes: Vec<Node>
}

pub fn get_topology() -> Result<Topology, Box<dyn std::error::Error>> {
    let mut res = reqwest::blocking::get("http://127.0.0.1:8081/v1/nes/topology").unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body)?;

    println!("Status: {}", res.status());
    println!("Headers:\n{:#?}", res.headers());
    println!("Body:\n{}", body);
    Ok(serde_json::from_str(&body)?)
}
