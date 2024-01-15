use crate::{constants::WHITE, tracers::SingleSphere, world::World};

impl World {
    pub fn build(&mut self) {
        self.view_plane.set_hres(200);
        self.view_plane.set_vres(200);
        self.view_plane.set_pixel_size(1.0);
        self.view_plane.set_gamma(1.0);
        self.view_plane.set_gamut_display(false);

        self.background_color = WHITE;

        let single_sphere = SingleSphere::new();
        let tracer = Box::new(single_sphere);
        self.set_tracer(tracer);

        self.sphere.set_center(0.0, 0.0, 0.0);
        self.sphere.set_radius(85.0);
    }
}
