//29.11.23 by Matteo Fava
//This file will contain the physics engine

//import from bodies
use crate::sim::bodies::Point;
use physical_constants::NEWTONIAN_CONSTANT_OF_GRAVITATION as G;

//ENGINE
pub struct Engine {
    bodies: Vec<Point>,
    t: f64,
    dt: f64,
}

impl Engine {
    pub fn new(dt: f64) -> Self {
        Self {bodies: Vec::new(), t: 0.0, dt}
    }

    pub fn add_point(&mut self, pos: (f64, f64, f64), v: (f64, f64, f64), mass: f64, charge: f64) {
            self.bodies.push(Point::new(pos, v, mass, charge))
    }

    fn calc_forces(&mut self) {
        
        for i in 0..self.bodies.len(){
            for j in i+1..self.bodies.len(){
                
            }
        }
    }

    fn calc_speeds(&mut self) {}

    fn calc_positions(&mut self) {}

    fn new_frame(&mut self) {}

    pub fn render_to_terminal(&self, every: f64) {}
    
    pub fn start(&mut self, end_in_s: f64) {}

}