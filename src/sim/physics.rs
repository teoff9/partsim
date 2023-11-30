//30.11.23 by Matteo Fava
//This file will contain physics equations

//imports
use crate::sim::bodies::Point;
use crate::sim::cartesian::Vector;
use physical_constants::{NEWTONIAN_CONSTANT_OF_GRAVITATION as G, VACUUM_ELECTRIC_PERMITTIVITY as epsilon};
use std::f64::consts::PI;

//Newton universal law of gravitation for two points
//adds the calculated forces in p1.f and p2.f
pub fn universal_law_gravity (p1: &Point, p2: &Point) -> Vector{
    let fx = -G*p1.mass*p2.mass*(p1.pos.x-p2.pos.x)/((p1.pos.x-p2.pos.x).abs()).powf(3.0);
    let fy = -G*p1.mass*p2.mass*(p1.pos.y-p2.pos.y)/((p1.pos.y-p2.pos.y).abs()).powf(3.0);
    let fz = -G*p1.mass*p2.mass*(p1.pos.z-p2.pos.z)/((p1.pos.z-p2.pos.z).abs()).powf(3.0);
    
    Vector::new((fx, fy, fz))
}

//Coulomb Law for two points
//adds the calculated forces in p1.f and p2.f
pub fn coulomb_law (p1: &Point, p2: &Point) -> Vector {
    let fx = p1.charge*p2.charge*(p1.pos.x-p2.pos.x)/(((p1.pos.x-p2.pos.x).abs()).powf(3.0)*(4.0*PI*epsilon));
    let fy = p1.charge*p2.charge*(p1.pos.y-p2.pos.y)/(((p1.pos.y-p2.pos.y).abs()).powf(3.0)*(4.0*PI*epsilon));
    let fz = p1.charge*p2.charge*(p1.pos.z-p2.pos.z)/(((p1.pos.z-p2.pos.z).abs()).powf(3.0)*(4.0*PI*epsilon));

    Vector::new((fx,fy,fz))
}