//29.11.23 by Matteo Fava
//This file will contain Vector struct
//and related methods


//struct for Cartesian Vector
#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn new(v: (f64, f64, f64)) -> Self {
        Self {x: v.0, y: v.1, z: v.2}
    }

    pub fn sum(&mut self, vett: &Vector) {
        self.x += vett.x;
        self.y += vett.y;
        self.z += vett.z;
    }

    pub fn subtract(&mut self, vett: &Vector) {
        self.x -= vett.x;
        self.y -= vett.y;
        self.z -= vett.z;
    }
}
