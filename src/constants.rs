use crate::utilities::RGBColor;

pub const SCREEN_WIDTH: u32 = 800;
pub const SCREEN_HEIGHT: u32 = 600;

pub const PI: f64 = 3.1415926535897932384;
pub const TWO_PI: f64 = 6.2831853071795864769;
pub const PI_ON_180: f64 = 0.0174532925199432957;
pub const INV_PI: f64 = 0.3183098861837906715;
pub const INV_TWO_PI: f64 = 0.1591549430918953358;

pub const K_EPSILON: f64 = 0.0001;
pub const K_HUGE_VALUE: f64 = 1.0E10;

pub const BLACK: RGBColor = RGBColor::new(0.0, 0.0, 0.0);
pub const WHITE: RGBColor = RGBColor::new(1.0, 1.0, 1.0);
pub const RED: RGBColor = RGBColor::new(1.0, 0.0, 0.0);
