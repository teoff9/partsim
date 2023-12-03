//29.11.23 by Matteo Fava
//Main code of partsim

//import the engine
pub mod sim;
use crate::sim::engine::Engine;

fn main() {
    let mut sim1 = Engine::new(1.0 / 1000000.0);
    sim1.add_point((100.0, 0.0, 0.0), (10.0, 0.0, 0.0), 100.0, 0.0);
    sim1.add_point(
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        1.0f64 * 10.0f64.powf(15.0),
        0.0,
    );

    sim1.start(3.5, 50000.0);
}
