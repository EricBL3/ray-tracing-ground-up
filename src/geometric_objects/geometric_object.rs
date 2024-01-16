use crate::utilities::{RGBColor, Ray, ShadeRec};

pub trait GeometricObject {
    fn hit(&self, ray: &Ray, t_min: &mut f64, s: &mut ShadeRec) -> bool;

    fn get_color(&self) -> RGBColor;

    fn set_color(&mut self, r: f32, g: f32, b: f32);

    fn set_color_from_rgb_color(&mut self, color: RGBColor);
}
