//29.11.23 by Matteo Fava
//This file will contain the physics engine

//imports
use super::bodies::Point;
use super::cartesian::Vector;
use super::physics::{partial_coulomb, partial_law_gravity};

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
                self.fcache.push((
                    [i, j],
                    partial_law_gravity(&self.bodies[i].mass, &self.bodies[j].mass)
                        + partial_coulomb(&self.bodies[i].charge, &self.bodies[j].charge),
                ));
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
            f.times_costant(*pf / f.module().powf(3.0));

            self.bodies[i[0]].f.add(&f);
            self.bodies[i[1]].f.subtract(&f);
        }
    }

    //updates the speeds of each body
    fn new_speeds(&mut self) {
        for b in &mut self.bodies {
            b.v.add_tuple((
                b.f.x / b.mass * self.dt,
                b.f.y / b.mass * self.dt,
                b.f.z / b.mass * self.dt,
            ));
        }
    }

    //updates positions of each body approximating movement for dt
    //as an accellerated motion along a line
    fn update_positions(&mut self) {
        for b in &mut self.bodies {
            b.pos.add_tuple((
                b.v.x * self.dt + 0.5 * self.dt.powf(2.0) * b.f.x / b.mass,
                b.v.y * self.dt + 0.5 * self.dt.powf(2.0) * b.f.y / b.mass,
                b.v.z * self.dt + 0.5 * self.dt.powf(2.0) * b.f.z / b.mass,
            ));
        }
    }

    //show bodies positions in the terminal
    fn render_to_terminal(&self, i: f64) {
        println!("Time: {:.2}", (i * self.dt));
        for (i, b) in self.bodies.iter().enumerate() {
            println!(
                "Body {} position: ( {}, {}, {} )",
                i, b.pos.x, b.pos.y, b.pos.z
            );
        }
        println!("r % = {} %", (self.bodies[0].distance_from(&self.bodies[1]) - 42164.0 * 10.0f64.powf(3.0))* 100.0 / 42164.0 * 10.0f64.powf(3.0));
        //println!("R % = {} %", (self.bodies[1].distance_from(&self.bodies[2]) - 150.0 * 10.0f64.powf(9.0))* 100.0 / 150.0 * 10.0f64.powf(9.0));
        println!();
    }

    //start simulation for t = final_t seconds
    pub fn start_for(&mut self, final_t: f64, show_every_n_dt: f64) {
        let mut i = 0.0;
        let mut j = 0.0;

        self.calc_fcache();

        self.render_to_terminal(i);

        while i * self.dt < final_t {
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
