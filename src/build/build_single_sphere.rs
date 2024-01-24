use crate::{constants::WHITE, tracers::SingleSphere, world::*};

use super::Builder;

pub struct BuildSingleSphere;

impl Builder for BuildSingleSphere {
    fn build(&self, world: &mut World) {
        world.view_plane.set_hres(400);
        world.view_plane.set_vres(400);
        world.view_plane.set_pixel_size(1.0);
        world.view_plane.set_gamma(1.0);
        world.view_plane.set_gamut_display(false);
        world.view_plane.set_num_samples(9);

        world.background_color = WHITE;

        let single_sphere = SingleSphere::new();
        let tracer = Box::new(single_sphere);
        world.set_tracer(tracer);

        world.sphere.set_center(-200.0, 100.0, 0.0);
        world.sphere.set_radius(300.0);
    }
}
