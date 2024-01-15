mod build;
mod constants;
mod geometric_objects;
mod ray_tracer_window;
mod tracers;
mod utilities;
mod world;

use constants::BLACK;
use geometric_objects::Sphere;
use nalgebra::Point3;
use ray_tracer_window::RayTracerWindow;
use world::{ViewPlane, World};

pub fn main() {
    // let mut i = 0;
    // while !window.should_close() {
    //     i = (i + 1) % 255;
    //     window.set_pixel(100, 100, i, 64, 255 - i);

    //     ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // }

    let vp = ViewPlane::new(0, 0, 0.0, 0.0, 0.0, false);
    let sphere = Sphere::new(Point3::new(0.0, 0.0, 0.0), 0.0);
    let window = RayTracerWindow::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT);

    let mut w = World::new(vp, BLACK, sphere, window);
    w.build();
    w.render_scene();
}
