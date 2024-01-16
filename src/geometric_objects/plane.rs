use nalgebra::{Point3, Vector3};

use crate::{
    constants::{BLACK, K_EPSILON},
    utilities::{RGBColor, Ray, ShadeRec},
};

use super::GeometricObject;

#[derive(Clone, Default)]
pub struct Plane {
    point: Point3<f64>,
    normal: Vector3<f64>,
    color: RGBColor,
}

impl Plane {
    pub fn new(point: Point3<f64>, normal: Vector3<f64>) -> Self {
        Self {
            point,
            normal,
            color: BLACK,
        }
    }
}

impl GeometricObject for Plane {
    fn hit(&self, ray: &Ray, t_min: &mut f64, sr: &mut ShadeRec) -> bool {
        let t = (self.point - ray.origin).dot(&self.normal) / (ray.direction.dot(&self.normal));

        if t > K_EPSILON {
            *t_min = t;
            sr.normal = self.normal;
            sr.local_hit_point = ray.origin + (t * ray.direction);

            return true;
        }

        false
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.color = RGBColor::new(r, g, b);
    }

    fn set_color_from_rgb_color(&mut self, color: RGBColor) {
        self.color = color;
    }

    fn get_color(&self) -> RGBColor {
        self.color
    }
}
