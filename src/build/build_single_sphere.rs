use std::{cell::RefCell, rc::Rc};

use crate::{constants::WHITE, tracers::SingleSphere, world::World};

impl World {
    pub fn build(&mut self) {
        self.view_plane.set_hres(200);
        self.view_plane.set_vres(200);
        self.view_plane.set_pixel_size(1.0);
        self.view_plane.set_gamma(1.0);

        self.background_color = WHITE;

        let world_rc = Rc::new(RefCell::new(std::mem::take(self)));
        let single_sphere = SingleSphere::new(world_rc.clone());
        let tracer = Box::new(single_sphere);
        self.set_tracer(tracer);

        println!("Set Tracer");

        self.sphere.set_center(0.0, 0.0, 0.0);
        self.sphere.set_radius(85.0);

        println!("Set Sphere");
    }
}
