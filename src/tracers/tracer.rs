use crate::{constants::*, utilities::*, world::World};

pub trait Tracer {
    fn trace_ray(&self, _world: &World, _ray: &Ray) -> RGBColor {
        return BLACK;
    }

    fn trace_ray_with_depth(&self, _ray: &Ray, _depth: i32) -> RGBColor {
        return BLACK;
    }
}
