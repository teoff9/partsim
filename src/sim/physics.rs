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
pub fn universal_law_gravity(p1: &Point, p2: &Point) -> Vector {
    let f = -G * p1.mass * p2.mass / p1.distance_from(p2).powf(3.0);

    Vector::new((
        f * (p1.pos.x - p2.pos.x),
        f * (p1.pos.y - p2.pos.y),
        f * (p1.pos.z - p2.pos.z),
    ))
}

//Coulomb Law for two points
pub fn coulomb_law(p1: &Point, p2: &Point) -> Vector {
    let f = p1.charge * p2.charge / (4.0 * PI * epsilon0 * p1.distance_from(p2).powf(3.0));

    Vector::new((
        f * (p1.pos.x - p2.pos.x),
        f * (p1.pos.y - p2.pos.y),
        f * (p1.pos.z - p2.pos.z),
    ))
}
