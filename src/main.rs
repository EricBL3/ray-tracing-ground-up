mod constants;
mod geometric_objects;
mod ray_tracer_window;
mod utilities;
mod world;

use ray_tracer_window::RayTracerWindow;
use std::time::Duration;

pub fn main() {
    let mut window = RayTracerWindow::new(constants::SCREEN_WIDTH, constants::SCREEN_HEIGHT);

    let mut i = 0;
    while !window.should_close() {
        i = (i + 1) % 255;
        window.set_pixel(100, 100, i, 64, 255 - i);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
