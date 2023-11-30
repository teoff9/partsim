//29.11.23 by Matteo Fava
//Main code of partsym

use crate::sim::engine::Engine;
pub mod sim;

fn main() {
    let mut sim1 = Engine::new(0.01);
    sim1.add_point((10.0, 0.0, 0.0), (0.0,0.0,0.0), 1.0, 0.0);
    sim1.add_point((-10.0, 0.0, 0.0), (0.0,0.0,0.0), 1.0, 0.0);

    


    
}
