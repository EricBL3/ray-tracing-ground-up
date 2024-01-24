use crate::{
    utilities::{RGBColor, Ray},
    world::World,
};

use super::Tracer;

pub struct Sinusoid {}

impl Sinusoid {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tracer for Sinusoid {
    fn trace_ray(&self, _world: &World, _ray: &Ray) -> RGBColor {
        let col_val =
            0.5 * (1.0 + (_ray.origin.x.powf(2.0) as f32 * _ray.origin.y.powf(2.0) as f32).sin());

        return RGBColor::new(col_val, col_val, col_val);
    }
}
