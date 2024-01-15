use crate::utilities::{Ray, ShadeRec};

pub trait GeometricObject {
    fn hit(&self, ray: &Ray, t_min: &mut f64, s: &mut ShadeRec) -> bool;
}
