#[derive(Clone, Default)]
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
