//29.11.23 by Matteo Fava
//This file will contain the physics engine

//imports
use crate::sim::bodies::Point;
use crate::sim::physics::{universal_law_gravity, coulomb_law};
use crate::sim::cartesian::Vector;

//ENGINE
pub struct Engine {
    bodies: Vec<Point>,
    t: f64,
    dt: f64,
}

impl Engine {

    //creates new instance of Engine
    //dt is the refresh time interval for the simulation
    pub fn new(dt: f64) -> Self {
        Self {bodies: Vec::new(), t: 0.0, dt}
    }

    //add a material point (Point) to self.bodies
    pub fn add_point(&mut self, pos: (f64, f64, f64), v: (f64, f64, f64), mass: f64, charge: f64) {
            if mass > 0.0 {
                self.bodies.push(Point::new(pos, v, mass, charge));
            } else {
                panic!("A material point should always have positive mass!");
            } 
    }

    //Calculates the total forces on each body: Newton Grav. Law and Coulomb Law
    fn calc_forces(&mut self) {
        //set all f's to 0.0
        for i in 0..self.bodies.len(){
            self.bodies[i].f = Vector::new((0.0,0.0,0.0));
        }
        
        //calculate forces for each bodies combinations
        for i in 0..self.bodies.len(){
            for j in i+1..self.bodies.len(){
                //calculate gravitational forces
                let f = universal_law_gravity(&self.bodies[i], &self.bodies[j]);
                self.bodies[i].f.sum(&f);
                self.bodies[j].f.subtract(&f);
                
                //if they both have a charge, add coulomb force
                if self.bodies[i].charge != 0.0 && self.bodies[j].charge != 0.0 {
                    let f = coulomb_law(&self.bodies[i], &self.bodies[j]);
                    self.bodies[i].f.sum(&f);
                    self.bodies[j].f.subtract(&f);
                }
            }
        }
    }

    fn new_speeds(&mut self) {}

    fn calc_positions(&mut self) {}

    fn new_frame(&mut self) {}

    fn render_to_terminal(&self, every: f64) {}
    
    pub fn start(&mut self, end_in_s: f64, show_every: f64) {
    }

}