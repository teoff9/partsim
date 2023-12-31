//29.11.23 by Matteo Fava
//This file will contain Vector struct
//and related methods

//struct for Cartesian Vector
#[derive(Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(v: (f64, f64, f64)) -> Self {
        Self {
            x: v.0,
            y: v.1,
            z: v.2,
        }
    }

    pub fn from_subtraction(v1: &Vector, v2: &Vector) -> Self {
        Self {
            x: v1.x - v2.x,
            y: v1.y - v2.y,
            z: v1.z - v2.z,
        }
    }

    pub fn from_addiction(v1: &Vector, v2: &Vector) -> Self {
        Self {
            x: v1.x + v2.x,
            y: v1.y + v2.y,
            z: v1.z + v2.z,
        }
    }

    pub fn add(&mut self, vett: &Vector) {
        self.x += vett.x;
        self.y += vett.y;
        self.z += vett.z;
    }

    pub fn subtract(&mut self, vett: &Vector) {
        self.x -= vett.x;
        self.y -= vett.y;
        self.z -= vett.z;
    }

    pub fn add_tuple(&mut self, vett: (f64, f64, f64)) {
        self.x += vett.0;
        self.y += vett.1;
        self.z += vett.2;
    }

    pub fn subtract_tuple(&mut self, vett: (f64, f64, f64)) {
        self.x -= vett.0;
        self.y -= vett.1;
        self.z -= vett.2;
    }

    pub fn times_costant(&mut self, cost: f64) {
        self.x *= cost;
        self.y *= cost;
        self.z *= cost;
    }

    pub fn divide_by_costant(&mut self, cost: f64) {
        self.x /= cost;
        self.y /= cost;
        self.z /= cost;
    }

    pub fn module(&self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).powf(0.5)
    }

    pub fn set_values(&mut self, new: (f64, f64, f64)) {
        self.x = new.0;
        self.y = new.1;
        self.z = new.2;
    }
}
