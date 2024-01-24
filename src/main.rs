mod build;
mod constants;
mod geometric_objects;
mod ray_tracer_window;
mod tracers;
mod utilities;
mod world;

use constants::BLACK;
use world::World;

use crate::build::*;

pub fn main() {
    let mut w = World::new(BLACK);
    println!("Building world...");
    w.build(BuildSingleSphere);
    if w.tracer.is_none() {
        println!("ERROR: The tracer was not specified in the build function");
    } else {
        w.render_scene();
    }
}
