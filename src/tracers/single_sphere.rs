use sdl2::pixels::Color;

use crate::{geometric_objects::GeometricObject, utilities::ShadeRec, world::World};

use super::Tracer;

pub struct SingleSphere {
    world: World,
}

impl SingleSphere {
    pub fn new(&self, world: World) -> Self {
        Self { world: world }
    }
}

impl Tracer for SingleSphere {
    fn trace_ray(&self, _ray: &crate::utilities::Ray) -> Color {
        let mut sr = ShadeRec::new(&self.world);
        let mut t = 0.0;

        if self.world.sphere.hit(_ray, &mut t, &mut sr) {
            return Color::RED;
        } else {
            return Color::BLACK;
        }
    }
}
