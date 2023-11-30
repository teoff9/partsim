//29.11.23 by Matteo Fava
//This file will contain Vett struct
//and related methods

pub struct Vector {
    x: f64,
    y: f64,
    z: f64
}

impl Vector {
    pub fn new(v: (f64, f64, f64)) -> Self {
        Self {x: v.0, y: v.1, z: v.2}
    }
}
