extern crate nalgebra as na;

use na::{Point3, Vector3};

use crate::{constants::BLACK, world::*};

use super::RGBColor;

pub struct ShadeRec<'a> {
    pub hit_an_object: bool,
    pub local_hit_point: Point3<f64>,
    pub normal: Vector3<f64>,
    pub color: RGBColor,
    pub world: &'a World,
}

impl<'a> ShadeRec<'a> {
    pub fn new(world: &'a World) -> Self {
        Self {
            hit_an_object: false,
            local_hit_point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            color: BLACK,
            world: &world,
        }
    }
}
