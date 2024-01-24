use crate::{constants::BLACK, tracers::Sinusoid, world::*};

use super::Builder;

pub struct BuildSinusoid;

impl Builder for BuildSinusoid {
    fn build(&self, world: &mut World) {
        world.view_plane.set_hres(512);
        world.view_plane.set_vres(512);
        world.view_plane.set_pixel_size(0.02);
        world.view_plane.set_num_samples(25);

        world.background_color = BLACK;

        let tracer = Box::new(Sinusoid::new());
        world.set_tracer(tracer);
    }
}
