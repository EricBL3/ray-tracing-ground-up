use nalgebra::Point3;

use crate::{constants::BLACK, geometric_objects::Sphere, tracers::MultipleObjects, world::*};

use super::Builder;

pub struct Build2SpheresAndPlane;

impl Builder for Build2SpheresAndPlane {
    fn build(&self, world: &mut World) {
        world.view_plane.set_hres(200);
        world.view_plane.set_vres(200);

        world.background_color = BLACK;

        let mut sphere = Sphere::new(Point3::new(0.0, -25.0, 0.0), 80.0);
        sphere.set_color(1.0, 0.0, 0.0);
        world.add_object(Box::new(sphere));

        sphere = Sphere::new(Point3::new(0.0, 30.0, 0.0), 60.0);
        sphere.set_color(1.0, 1.0, 0.0);
        world.add_object(Box::new(sphere));

        let tracer = Box::new(MultipleObjects::new());
        world.set_tracer(tracer);
    }
}
