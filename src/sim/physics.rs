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
    let f = -G * p1.mass * p2.mass
        / Vector::from_subtraction(&p1.pos, &p2.pos)
            .module()
            .powf(3.0);

    Vector::new((
        f * (p1.pos.x - p2.pos.x),
        f * (p1.pos.y - p2.pos.y),
        f * (p1.pos.z - p2.pos.z),
    ))
}

//Coulomb Law for two points
//adds the calculated forces in p1.f and p2.f
pub fn coulomb_law(p1: &Point, p2: &Point) -> Vector {
    let f = p1.charge * p2.charge
        / (4.0
            * PI
            * epsilon0
            * Vector::from_subtraction(&p1.pos, &p2.pos)
                .module()
                .powf(3.0));

    Vector::new((
        f * (p1.pos.x - p2.pos.x),
        f * (p1.pos.y - p2.pos.y),
        f * (p1.pos.z - p2.pos.z),
    ))
}
