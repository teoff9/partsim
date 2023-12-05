//29.11.23 by Matteo Fava
//This file will contain structs of bodies
//such as Point (material point), Sphere ...

//imports
use crate::sim::cartesian::Vector;


//MATERIAL POINT
pub struct Point {
    pub pos: Vector,
    pub v: Vector,
    pub f: Vector,
    pub mass: f64,
    pub charge: f64,
}


impl Point {

    //returns new instance of Point taking tuples of f64
    pub fn new(pos: (f64, f64, f64), v: (f64, f64, f64), mass: f64, charge: f64) -> Self {
        if mass < 0.0 {
            panic!("Mass cannot be less than 0...\n");
        } else {
            let p = Vector::new(pos);
            let vv = Vector::new(v);
            let f = Vector::new((0.0, 0.0, 0.0));
            Self {
                pos: p,
                v: vv,
                f,
                mass,
                charge,
            }
        }
    }

    //Returns the distance from anoter point p
    pub fn distance_from(&self, p: &Point) -> f64 {
        ((self.pos.x - p.pos.x).powf(2.0)
            + (self.pos.y - p.pos.y).powf(2.0)
            + (self.pos.z - p.pos.z).powf(2.0))
        .powf(0.5)
    }
}
