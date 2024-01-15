mod build;
mod constants;
mod geometric_objects;
mod ray_tracer_window;
mod tracers;
mod utilities;
mod world;

use constants::BLACK;
use world::World;

pub fn main() {
    let mut w = World::new(BLACK);
    println!("Building world...");
    w.build();
    println!("Rendering scene...");
    w.render_scene();
    println!("Done!");
}
