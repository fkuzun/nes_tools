use nes_tools::topology;
use nes_tools::topology::get_topology;

fn main() {
    let topology = get_topology();
    dbg!(&topology);

}
