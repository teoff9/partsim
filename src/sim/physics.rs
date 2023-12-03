//30.11.23 by Matteo Fava
//This file will contain physics equations

//imports
use crate::sim::bodies::Point;
use crate::sim::cartesian::Vector;
use physical_constants::{
    NEWTONIAN_CONSTANT_OF_GRAVITATION as G, VACUUM_ELECTRIC_PERMITTIVITY as epsilon0,
};
use std::f64::consts::PI;

//Newton universal law of gravitation for two points
//adds the calculated forces in p1.f and p2.f
pub fn universal_law_gravity(p1: &Point, p2: &Point) -> Vector {
    let f = -G * p1.mass * p2.mass;

    let fx = f * {
        if p1.pos.x != 0.0 || p2.pos.x != 0.0 {
            (p1.pos.x - p2.pos.x) / ((p1.pos.x - p2.pos.x).abs()).powf(3.0)
        } else {
            0.0
        }
    };
    let fy = f * {
        if p1.pos.y != 0.0 || p2.pos.y != 0.0 {
            (p1.pos.y - p2.pos.y) / ((p1.pos.y - p2.pos.y).abs()).powf(3.0)
        } else {
            0.0
        }
    };
    let fz = f * {
        if p1.pos.z != 0.0 || p2.pos.z != 0.0 {
            (p1.pos.z - p2.pos.z) / ((p1.pos.z - p2.pos.z).abs()).powf(3.0)
        } else {
            0.0
        }
    };

    Vector::new((fx, fy, fz))
}

//Coulomb Law for two points
//adds the calculated forces in p1.f and p2.f
pub fn coulomb_law(p1: &Point, p2: &Point) -> Vector {
    let f = p1.charge * p2.charge / (4.0 * PI * epsilon0);

    Vector::new((
        f * (p1.pos.x - p2.pos.x) / ((p1.pos.x - p2.pos.x).abs()).powf(3.0),
        f * (p1.pos.y - p2.pos.y) / ((p1.pos.y - p2.pos.y).abs()).powf(3.0),
        f * (p1.pos.z - p2.pos.z) / ((p1.pos.z - p2.pos.z).abs()).powf(3.0),
    ))
}
