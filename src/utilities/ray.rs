use nalgebra::{Point3, Vector3};

#[derive(Clone)]
pub struct Ray {
    origin: Point3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Point3<f64>, dir: Vector3<f64>) -> Self {
        Self {
            origin: origin,
            direction: dir,
        }
    }
}
