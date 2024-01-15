use crate::{
    constants::*,
    geometric_objects::GeometricObject,
    utilities::{RGBColor, Ray, ShadeRec},
    world::World,
};

use super::Tracer;

pub struct SingleSphere {}

impl SingleSphere {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tracer for SingleSphere {
    fn trace_ray(&self, _world: &World, _ray: &Ray) -> RGBColor {
        let mut sr = ShadeRec::new();
        let mut t = 0.0;

        if _world.sphere.hit(_ray, &mut t, &mut sr) {
            return RED;
        } else {
            return BLACK;
        }
    }
}
