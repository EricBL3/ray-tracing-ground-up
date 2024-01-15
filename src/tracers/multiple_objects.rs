use crate::{
    utilities::{RGBColor, Ray},
    world::World,
};

use super::Tracer;

pub struct MultipleObjects {}

impl MultipleObjects {
    pub fn new() -> Self {
        Self {}
    }
}

impl Tracer for MultipleObjects {
    fn trace_ray(&self, _world: &World, _ray: &Ray) -> RGBColor {
        let sr = _world.hit_bare_bones_objects(_ray);

        if sr.hit_an_object {
            return sr.color;
        } else {
            return _world.background_color;
        }
    }
}
