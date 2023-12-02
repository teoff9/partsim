//29.11.23 by Matteo Fava
//Main code of partsim

//import the engine
pub mod sim;
use crate::sim::engine::Engine;

fn main() {
    let mut sim1 = Engine::new(0.01);
    sim1.add_point((10.0, 0.0, 0.0), (0.0,0.0,0.0), 1000.0, 0.0);
    sim1.add_point((-10.0, 0.0, 0.0), (0.0,0.0,0.0), 1000.0, 0.0);
    sim1.add_point((0.0,0.0,0.0), (0.0,0.0,0.0), 1000.0, 0.0);


    


    


    
}
