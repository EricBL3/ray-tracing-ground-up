extern crate nalgebra as na;

use na::{Point3, Vector3};
use sdl2::pixels::Color;

use crate::world::*;

#[derive(Clone)]
pub struct ShadeRec {
    hit_an_object: bool,
    local_hit_point: Point3<f64>,
    normal: Vector3<f64>,
    color: Color,
    world: World,
}

impl ShadeRec {
    pub fn new(world: World) -> Self {
        Self {
            hit_an_object: false,
            local_hit_point: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            color: Color::BLACK,
            world: world,
        }
    }
}
