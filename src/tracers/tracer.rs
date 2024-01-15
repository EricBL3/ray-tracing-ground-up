use crate::{constants::*, utilities::*};

pub trait Tracer {
    fn trace_ray(&self, _ray: &Ray) -> RGBColor {
        return BLACK;
    }

    fn trace_ray_with_depth(&self, _ray: &Ray, _depth: i32) -> RGBColor {
        return BLACK;
    }
}
