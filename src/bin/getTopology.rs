use nes_rest::topology;
use nes_rest::topology::get_topology;

fn main() {
    let topology = get_topology();
    dbg!(&topology);

}