//29.11.23 by Matteo Fava
//This file will contain the physics engine

//imports
use crate::sim::bodies::Point;
use crate::sim::physics::{universal_law_gravity, coulomb_law};
use crate::sim::cartesian::Vector;

//ENGINE
pub struct Engine {
    bodies: Vec<Point>,
    dt: f64,
}

impl Engine {

    //creates new instance of Engine
    //dt is the refresh time interval for the simulation
    pub fn new(dt: f64) -> Self {
        Self {bodies: Vec::new(), dt}
    }

    //add a material point (Point) to self.bodies
    pub fn add_point(&mut self, pos: (f64, f64, f64), v: (f64, f64, f64), mass: f64, charge: f64) {
        self.bodies.push(Point::new(pos, v, mass, charge));

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

    fn new_speeds(&mut self) {
        for i in 0..self.bodies.len(){
            
            let new_v = Vector::new((
                self.bodies[i].f.x/self.bodies[i].mass * self.dt,
                self.bodies[i].f.y/self.bodies[i].mass * self.dt,
                self.bodies[i].f.z/self.bodies[i].mass * self.dt
            ));
            self.bodies[i].v.sum(&new_v);
        }
    }

    fn update_positions(&mut self) {
        for i in 0..self.bodies.len(){

            let s = Vector::new((
                self.bodies[i].v.x*self.dt + 0.5*self.dt.powf(2.0)*self.bodies[i].f.x/self.bodies[i].mass, 
                self.bodies[i].v.y*self.dt + 0.5*self.dt.powf(2.0)*self.bodies[i].f.y/self.bodies[i].mass,
                self.bodies[i].v.z*self.dt  + 0.5*self.dt.powf(2.0)*self.bodies[i].f.z/self.bodies[i].mass));
            self.bodies[i].pos.sum(&s);
        }
    }

    fn render_to_terminal(&self) {
        for i in 0..self.bodies.len(){
            println!("Body {} position: ( {}, {}, {} )", 
                i, self.bodies[i].pos.x, self.bodies[i].pos.y, self.bodies[i].pos.z);
        }
        println!("R = {}", ((self.bodies[0].pos.x-self.bodies[1].pos.x).powf(2.0)+(self.bodies[0].pos.y-self.bodies[1].pos.y).powf(2.0)).powf(0.5));
        println!("");
    }
    
    pub fn start(&mut self, end_in_s: f64, show_every_n_dt: f64) {
    
        let mut i = 0.0;
        let mut j = 0.0;
        self.render_to_terminal();
        while i*self.dt < end_in_s {
            self.calc_forces();
            self.update_positions();
            self.new_speeds();
            i += 1.0;
            j += 1.0;

            if j == show_every_n_dt {
                self.render_to_terminal();
                j = 0.0;
            }
        }
    }

}