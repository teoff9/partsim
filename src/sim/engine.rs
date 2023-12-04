//29.11.23 by Matteo Fava
//This file will contain the physics engine

//imports
use crate::sim::bodies::Point;
use crate::sim::cartesian::Vector;
use crate::sim::physics::{
     partial_coulomb, partial_law_gravity
};

//ENGINE
pub struct Engine {
    bodies: Vec<Point>,
    dt: f64,
    fcache: Vec<([usize; 2], f64)>,
}

impl Engine {
    //creates new instance of Engine
    //dt is the refresh time interval for the simulation
    pub fn new(dt: f64) -> Self {
        Self {
            bodies: vec![],
            dt,
            fcache: vec![],
        }
    }

    //add a material point (Point) to self.bodies
    pub fn add_point(&mut self, pos: (f64, f64, f64), v: (f64, f64, f64), mass: f64, charge: f64) {
        self.bodies.push(Point::new(pos, v, mass, charge));
    }

    //function to generate fcache
    fn calc_fcache(&mut self) {
        for i in 0..self.bodies.len() {
            for j in i + 1..self.bodies.len() {
                self.fcache.push(
                    ([i, j],
                    partial_law_gravity(&self.bodies[i].mass, &self.bodies[j].mass)
                        + partial_coulomb(&self.bodies[i].charge, &self.bodies[j].charge)),
                );
            }
        }
    }

    //Calculates the total forces on each body: Newton Grav. Law and Coulomb Law
    fn calc_forces(&mut self) {
        
        //set all f's to 0.0
        for b in &mut self.bodies {
            b.f.set_values((0.0, 0.0, 0.0));
        }

        //calculate forces for each bodies combinations
        for (i, pf) in &self.fcache {
            let mut f = Vector::from_subtraction(&self.bodies[i[0]].pos, &self.bodies[i[1]].pos);
            f.times_costant(*pf/f.module().powf(3.0));

            self.bodies[i[0]].f.add(&f);
            self.bodies[i[1]].f.subtract(&f);
        }
    }

    fn new_speeds(&mut self) {
        for i in 0..self.bodies.len() {
            let new_v = Vector::new((
                self.bodies[i].f.x / self.bodies[i].mass * self.dt,
                self.bodies[i].f.y / self.bodies[i].mass * self.dt,
                self.bodies[i].f.z / self.bodies[i].mass * self.dt,
            ));
            self.bodies[i].v.add(&new_v);
        }
    }

    fn update_positions(&mut self) {
        for i in 0..self.bodies.len() {
            let s = Vector::new((
                self.bodies[i].v.x * self.dt
                    + 0.5 * self.dt.powf(2.0) * self.bodies[i].f.x / self.bodies[i].mass,
                self.bodies[i].v.y * self.dt
                    + 0.5 * self.dt.powf(2.0) * self.bodies[i].f.y / self.bodies[i].mass,
                self.bodies[i].v.z * self.dt
                    + 0.5 * self.dt.powf(2.0) * self.bodies[i].f.z / self.bodies[i].mass,
            ));
            self.bodies[i].pos.add(&s);
        }
    }

    fn render_to_terminal(&self, i: f64) {
        println!("Time: {:.2}", (i * self.dt));
        for i in 0..self.bodies.len() {
            println!(
                "Body {} position: ( {}, {}, {} )",
                i, self.bodies[i].pos.x, self.bodies[i].pos.y, self.bodies[i].pos.z
            );
        }
        println!(
            "R = {}",
            ((self.bodies[0].pos.x - self.bodies[1].pos.x).powf(2.0)
                + (self.bodies[0].pos.y - self.bodies[1].pos.y).powf(2.0))
            .powf(0.5)
        );
        println!();
    }

    pub fn start(&mut self, end_in_s: f64, show_every_n_dt: f64) {
        let mut i = 0.0;
        let mut j = 0.0;
        
        self.calc_fcache();

        self.render_to_terminal(i);

        while i * self.dt < end_in_s {
            self.calc_forces();
            self.update_positions();
            self.new_speeds();

            i += 1.0;
            j += 1.0;

            if j >= show_every_n_dt {
                self.render_to_terminal(i);
                j = 0.0;
            }
        }
    }
}
