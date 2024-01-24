use super::ViewPlane;
use crate::build::Builder;
use crate::constants::*;
use crate::geometric_objects::Sphere;
use crate::geometric_objects::*;
use crate::ray_tracer_window::RayTracerWindow;
use crate::tracers::*;
use crate::utilities::*;
use nalgebra::Point2;
use nalgebra::{Point3, Vector3};
use rand::rngs::ThreadRng;
use rand::Rng;

pub struct World {
    pub view_plane: ViewPlane,
    pub background_color: RGBColor,
    pub sphere: Sphere,
    pub window: Option<RayTracerWindow>,
    pub tracer: Option<Box<dyn Tracer>>,
    pub objects: Vec<Box<dyn GeometricObject>>,
    pub rng_thread: ThreadRng,
}

impl World {
    pub fn new(background_color: RGBColor) -> Self {
        let view_plane = ViewPlane::default();
        let sphere = Sphere::default();        
        let mut rng = rand::thread_rng();

        Self {
            view_plane,
            background_color,
            sphere,
            window: None,
            tracer: None,
            objects: Vec::new(),
            rng_thread: rng
        }
    }

    pub fn set_tracer(&mut self, tracer: Box<dyn Tracer>) {
        self.tracer = Some(tracer);
    }

    pub fn build<T: Builder>(&mut self, builder: T) {
        builder.build(self);
    }

    pub fn render_scene(&mut self) {
        let h_res = self.view_plane.horizontal_res;
        let v_res = self.view_plane.vertical_res;
        let s = self.view_plane.pixel_size;
        let zw = 100.0;
        let n = (self.view_plane.num_samples as f32).sqrt() as u32;
        let mut sample_point = Point2::new(0.0, 0.0);

        self.window = Some(RayTracerWindow::new(h_res, v_res));

        let ray_origin = Point3::new(0.0, 0.0, 0.0);
        let ray_dir = Vector3::new(0.0, 0.0, -1.0);
        let mut ray = Ray::new(ray_origin, ray_dir);
        let mut quit = false;
        println!("Rendering scene...");
        // up
        for r in 0..v_res {
            // accross
            for c in 0..h_res {
                let mut pixel_color = BLACK;

                // up pixel
                for p in 0..n {
                    // across pixel
                    for q in 0..n {
                        sample_point.x = s
                            * (c as f64 - 0.5 * self.view_plane.horizontal_res as f64
                                + (q as f64 + self.rng_thread.gen::<f64>()) / n as f64);

                        sample_point.y = s
                            * (r as f64 - 0.5 * self.view_plane.vertical_res as f64
                                + (p as f64 + self.rng_thread.gen::<f64>()) / n as f64);

                        ray.origin = Point3::new(sample_point.x, sample_point.y, zw);
                        pixel_color += self.tracer.as_ref().unwrap().trace_ray(&self, &ray);
                    }
                }

                pixel_color /= self.view_plane.num_samples as f32; // avg the colors
                self.display_pixel(r, c, &pixel_color);

                if self.window.as_ref().unwrap().should_close() {
                    quit = true;
                }

                if quit {
                    break;
                }
            }
        }
        println!("Done!");
        //wait
        while !quit && !self.window.as_ref().unwrap().should_close() {}
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

        self.window.as_mut().unwrap().set_pixel(
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

    pub fn add_object(&mut self, object: Box<dyn GeometricObject>) {
        self.objects.push(object);
    }

    pub fn hit_bare_bones_objects(&self, ray: &Ray) -> ShadeRec {
        let mut sr = ShadeRec::new();
        let mut t = 0.0;
        let mut t_min = K_HUGE_VALUE;
        let num_objects = self.objects.len();

        for j in 0..num_objects {
            if self.objects[j].hit(ray, &mut t, &mut sr) && t < t_min {
                sr.hit_an_object = true;
                t_min = t;
                sr.color = self.objects[j].get_color();
            }
        }

        sr
    }
}
