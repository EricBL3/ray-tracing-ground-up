extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::render::*;
use sdl2::video::*;
use sdl2::Sdl;

pub struct RayTracerWindow {
    pub canvas: Canvas<Window>,
    sdl_context: Sdl,
}

impl RayTracerWindow {
    //Constructs and initializes the ray tracer window.
    pub fn new(width: u32, height: u32) -> Self {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Ray Tracing from the Ground Up", width, height)
            .position_centered()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        Self {
            canvas: canvas,
            sdl_context: sdl_context,
        }
    }

    //Function that draws a pixel of the specified RGB color at the specified location to the canvas.
    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        self.canvas.set_draw_color(Color::RGB(r, g, b));
        self.canvas.draw_point(Point::new(x, y)).unwrap();
        self.canvas.present();
    }

    //Determines if the window should close depending on user input.
    pub fn should_close(&self) -> bool {
        let mut event_pump = self.sdl_context.event_pump().unwrap();
        let mut result = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => result = true,
                _ => result = false,
            }
        }

        result
    }
}
