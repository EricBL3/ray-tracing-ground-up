use std::ops::{AddAssign, DivAssign};

#[derive(Clone, Default, Copy)]
pub struct RGBColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl RGBColor {
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    pub fn powc(&self, p: f32) -> RGBColor {
        return RGBColor::new(self.r.powf(p), self.g.powf(p), self.b.powf(p));
    }
}

impl AddAssign for RGBColor {
    fn add_assign(&mut self, other: Self) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl DivAssign<f32> for RGBColor {
    fn div_assign(&mut self, divisor: f32) {
        self.r /= divisor;
        self.g /= divisor;
        self.b /= divisor;
    }
}
