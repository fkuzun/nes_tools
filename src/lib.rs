use std::io::Read;

pub mod topology;
pub mod launch;
pub mod config;
pub mod query;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// pub fn get_mobile_nodes() -> () /*std::vec::Vec<>*/ {
//     let mut res =
//         reqwest::blocking::get("http://127.0.0.1:8081/v1/nes/location/allMobile").unwrap();
//     let mut body = String::new();
//     res.read_to_string(&mut body).unwrap();
//
//     println!("Status: {}", res.status());
//     println!("Headers:\n{:#?}", res.headers());
//     println!("Body:\n{}", body);
// }


#[cfg(test)]
mod tests {
    use crate::launch::{Launch, worker};
    use std::thread::sleep;
    use std::time::Duration;
    use crate::config::worker_config::{LocationProviderConfig, LocationProviderType, WorkerConfig, WorkerMobilityConfig};
    use crate::launch::coordinator;
    use crate::topology::NodeSpatialType;
    use super::*;

    //todo: use submodules for the testing binaries
    const COORDINATOR_PATH: &str = "/home/x/uni/ba/clion/nebulastream/cmake-build-debug-s2/nes-coordinator/nesCoordinator";
    const WORKER_PATH: &str = "/home/x/uni/ba/clion/nebulastream/cmake-build-debug-s2/nes-worker/nesWorker";
    const DUBLIN_BUS: &str = "/home/x/dublinBusScripts/trimmedOut.csv";
    const DUBLIN_BUS_CONFIG: &str = "/home/x/dublinBusScripts/dublinBusWorker.yaml";

    #[test]
    fn test_getting_topology() {
        let mut coordinator = coordinator::Coordinator::new(COORDINATOR_PATH);
        coordinator.launch().expect("TODO: panic message");

        let mut topology = None;
        while topology.is_none() {
            topology = dbg!(topology::get_topology().ok());
        }
        let mut topology = topology.unwrap();
        assert_eq!(topology.get_nodes().len(), 1);
        let mut worker = worker::Worker::new(WORKER_PATH, None);
        worker.launch().expect("TODO: panic message");

        //todo: add timeout
        while topology.get_nodes().len() == 1 {
            topology = topology::get_topology().ok().unwrap();
        }
        assert_eq!(topology.get_nodes().len(), 2);

        coordinator.kill().expect("TODO: panic message");
        worker.kill().expect("TODO: panic message");
    }

    #[test]
    fn test_getting_topology_with_client() {
        // let mut coordinator = coordinator::Coordinator::new(COORDINATOR_PATH);
        // coordinator.launch().expect("TODO: panic message");
        //
        // let client = reqwest::Client::new();
        // let mut topology = None;
        // while topology.is_none() {
        //     //topology = dbg!(topology::get_topology().ok());
        //     topology = dbg!(client.get("http://127.0.0.1:8081/v1/nes/topology").send().await.ok());
        // }
        // let mut topology = topology.unwrap();
        // assert_eq!(topology.get_nodes().len(), 1);
        // let mut worker = worker::Worker::new(WORKER_PATH, None);
        // worker.launch().expect("TODO: panic message");
        //
        // //todo: add timeout
        // while topology.get_nodes().len() == 1 {
        //     //topology = topology::get_topology().ok().unwrap();
        //     topology = dbg!(client.get("http://127.0.0.1:8081/v1/nes/topology").await.ok()).unwrap();
        // }
        // assert_eq!(topology.get_nodes().len(), 2);
        //
        // coordinator.kill().expect("TODO: panic message");
        // worker.kill().expect("TODO: panic message");
    }


    #[test]
    fn test_getting_mobile_nodes() {
        let mut coordinator = coordinator::Coordinator::new(COORDINATOR_PATH);
        coordinator.launch().expect("TODO: panic message");

        // let worker_config = WorkerConfig {
        //     worker_mobility_config: WorkerMobilityConfig {
        //         location_provider_config: LocationProviderConfig::new(DUBLIN_BUS),
        //         location_provider_type: LocationProviderType::CSV
        //     },
        //     node_spatial_type: NodeSpatialType::MOBILE_NODE,
        // };
        let mut worker = worker::Worker::from_config_path(WORKER_PATH, DUBLIN_BUS_CONFIG);
        //let mut worker = worker::Worker::new(WORKER_PATH, None);
        worker.launch().expect("TODO: panic message");

        let mut topology = None;
        while topology.is_none() {
            topology = dbg!(topology::get_topology().ok());
        }
        let mut topology = topology.unwrap();
        while topology.get_nodes().len() == 1 {
            topology = topology::get_topology().ok().unwrap();
        }

        assert_eq!(topology.get_nodes().len(), 2);
        let mut allMobile = topology::get_all_mobile().unwrap();
        assert_eq!(allMobile.nodes.len(), 1);

            coordinator.kill().expect("TODO: panic message");
        worker.kill().expect("TODO: panic message");
    }
}
