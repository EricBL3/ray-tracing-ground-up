use crate::utilities::{RGBColor, Ray, ShadeRec};

pub trait GeometricObject {
    fn hit(&self, ray: &Ray, t_min: &mut f64, s: &mut ShadeRec) -> bool;

    fn get_color(&self) -> RGBColor;
}
