use std::{cell::RefCell, rc::Rc};

use crate::{
    constants::*,
    geometric_objects::GeometricObject,
    utilities::{RGBColor, ShadeRec},
    world::World,
};

use super::Tracer;

pub struct SingleSphere {
    world: Rc<RefCell<World>>,
}

impl SingleSphere {
    pub fn new(world: Rc<RefCell<World>>) -> Self {
        Self { world: world }
    }
}

impl Tracer for SingleSphere {
    fn trace_ray(&self, _ray: &crate::utilities::Ray) -> RGBColor {
        let world = self.world.borrow();
        let mut sr = ShadeRec::new(&world);
        let mut t = 0.0;

        if world.sphere.hit(_ray, &mut t, &mut sr) {
            return RED;
        } else {
            return BLACK;
        }
    }
}
