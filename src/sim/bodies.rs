//29.11.23 by Matteo Fava
//This file will contain structs of bodies
//such as Point (material point), Sphere ...

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
}
