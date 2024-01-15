use core::f64;

use nalgebra::{Point3, Vector3};
use sdl2::pixels::Color;

use crate::{
    constants::K_EPSILON,
    utilities::{Ray, ShadeRec},
};

use super::GeometricObject;

#[derive(Clone)]
pub struct Sphere {
    center: Point3<f64>,
    radius: f64,
    color: Color,
}

impl Sphere {
    pub fn new(&self, c: Point3<f64>, r: f64) -> Self {
        Self {
            center: c,
            radius: r,
            color: Color::BLACK,
        }
    }

    pub fn set_color(&mut self, r: u8, g: u8, b: u8) {
        self.color = Color::RGB(r, g, b);
    }

    pub fn get_color(&self) -> Color {
        return self.color;
    }

    pub fn set_center(&mut self, x: f64, y: f64, z: f64) {
        self.center.x = x;
        self.center.y = y;
        self.center.z = z;
    }

    pub fn set_radius(&mut self, r: f64) {
        self.radius = r;
    }
}

impl GeometricObject for Sphere {
    fn hit(&self, ray: &Ray, t_min: &mut f64, sr: &mut ShadeRec) -> bool {
        let temp = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * temp.dot(&ray.direction);
        let c = temp.dot(&temp) - self.radius * self.radius;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            return false;
        } else {
            let e = disc.sqrt();
            let denom = 2.0 * a;
            let mut t = (-b - e) / denom; // smaller root

            if t > K_EPSILON {
                *t_min = t;
                sr.normal = (temp + (t * ray.direction)) / self.radius;
                sr.local_hit_point = ray.origin + (t * ray.direction);
                return true;
            }

            t = (-b + e) / denom; // larger root

            if t > K_EPSILON {
                *t_min = t;
                sr.normal = (temp + (t * ray.direction)) / self.radius;
                sr.local_hit_point = ray.origin + (t * ray.direction);
                return true;
            }
        }

        false
    }
}
