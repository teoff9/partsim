//29.11.23 by Matteo Fava
//Main code of partsim

//import the engine
pub mod sim;
use crate::sim::engine::Engine;

fn main() {
    let mut sim1 = Engine::new(1.0 / 100.0);
    
    sim1.add_point(
        (42164.0 * 10.0f64.powf(3.0), 0.0, 0.0),
        (0.0, 11000.0 / 3.6, 0.0),
        1.0,
        0.0,
    ); //satellite

    sim1.add_point(
        (0.0, 0.0, 0.0),
        (0.0, 0.0, 0.0),
        5.972f64 * 10.0f64.powf(24.0),
        0.0,
    ); //terra

    sim1.add_point(
        (150.0 * 10.0f64.powf(9.0), 0.0, 0.0),
        (0.0, 0.0, 0.0),
        4.0 * 10.0f64.powf(30.0),
        0.0,
    ); //sole

    sim1.start_for(30.0*24.0*3600.0, 10000.0);
}
