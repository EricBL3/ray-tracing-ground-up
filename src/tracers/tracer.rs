use sdl2::pixels::Color;

use crate::utilities::Ray;

pub trait Tracer {
    fn trace_ray(&self, _ray: &Ray) -> Color {
        return Color::BLACK;
    }

    fn trace_ray_with_depth(&self, _ray: &Ray, _depth: i32) -> Color {
        return Color::BLACK;
    }
}
