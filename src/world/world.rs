use super::super::RayTracerWindow;
use super::ViewPlane;
use crate::geometric_objects::Sphere;
use crate::tracers::*;
use crate::utilities::*;
use nalgebra::{Point3, Vector3};

#[derive(Default)]
pub struct World {
    pub view_plane: ViewPlane,
    pub background_color: RGBColor,
    pub sphere: Sphere,
    pub window: RayTracerWindow,
    pub tracer: Option<Box<dyn Tracer>>,
}

impl World {
    pub fn new(
        vp: ViewPlane,
        background_color: RGBColor,
        sphere: Sphere,
        window: RayTracerWindow,
    ) -> Self {
        Self {
            view_plane: vp,
            background_color: background_color,
            sphere: sphere,
            window: window,
            tracer: None,
        }
    }

    pub fn set_tracer(&mut self, tracer: Box<dyn Tracer>) {
        self.tracer = Some(tracer)
    }

    pub fn render_scene(&mut self) {
        let h_res = self.view_plane.horizontal_res;
        let v_res = self.view_plane.vertical_res;
        let s = self.view_plane.pixel_size;
        let zw = 100.0;

        let window = RayTracerWindow::new(h_res, v_res);

        let ray_origin = Point3::new(0.0, 0.0, 0.0);
        let ray_dir = Vector3::new(0.0, 0.0, -1.0);
        let mut ray = Ray::new(ray_origin, ray_dir);
        let mut quit = false;
        for r in 0..v_res {
            for c in 0..h_res {
                ray.origin = Point3::new(
                    s * (c as f64 - h_res as f64 / 2.0 + 0.5),
                    s * (r as f64 - v_res as f64 / 2.0 + 0.5),
                    zw,
                );

                if let Some(tracer_ptr) = &self.tracer {
                    let pixel_color = tracer_ptr.trace_ray(&ray);
                    self.display_pixel(r, c, &pixel_color);
                }

                if window.should_close() {
                    quit = true;
                }

                if quit {
                    break;
                }
            }
        }

        //wait
        while !quit && !window.should_close() {}
    }

    pub fn display_pixel(&mut self, row: u32, col: u32, raw_color: &RGBColor) {
        let mut mapped_color: RGBColor;
        if self.view_plane.show_out_of_gamut {
            mapped_color = self.clamp_to_color(raw_color);
        } else {
            mapped_color = self.max_to_one(raw_color);
        }

        if self.view_plane.gamma != 1.0 {
            mapped_color = mapped_color.powc(self.view_plane.inv_gamma);
        }

        let x = col;
        let y = self.view_plane.vertical_res - row - 1;

        self.window.set_pixel(
            x as i32,
            y as i32,
            (mapped_color.r * 255.0) as u8,
            (mapped_color.g * 255.0) as u8,
            (mapped_color.b * 255.0) as u8,
        );
    }

    pub fn max_to_one(&self, color: &RGBColor) -> RGBColor {
        let max_val = color.r.max(color.g.max(color.b));

        if max_val > 1.0 {
            return RGBColor::new(color.r / max_val, color.g / max_val, color.b / max_val);
        } else {
            return RGBColor::new(color.r, color.g, color.b);
        }
    }

    pub fn clamp_to_color(&self, raw_col: &RGBColor) -> RGBColor {
        let mut c = RGBColor::new(raw_col.r, raw_col.g, raw_col.b);

        if raw_col.r > 1.0 || raw_col.g > 1.0 || raw_col.b > 1.0 {
            c.r = 1.0;
            c.g = 0.0;
            c.b = 0.0;
        }

        c
    }
}
