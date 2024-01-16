use nalgebra::Point3;

use crate::{
    constants::BLACK, geometric_objects::*, tracers::MultipleObjects, utilities::RGBColor, world::*,
};

use super::Builder;

pub struct BuildChapter3Cover;

impl Builder for BuildChapter3Cover {
    fn build(&self, world: &mut World) {
        world.view_plane.set_hres(400);
        world.view_plane.set_vres(400);
        world.view_plane.set_pixel_size(0.5);

        world.background_color = BLACK;

        let tracer = Box::new(MultipleObjects::new());
        world.set_tracer(tracer);

        // Colors
        let yellow = RGBColor::new(1.0, 1.0, 0.0);
        let brown = RGBColor::new(0.71, 0.40, 0.16);
        let dark_green = RGBColor::new(0.0, 0.41, 0.41);
        let orange = RGBColor::new(1.0, 0.75, 0.0);
        let green = RGBColor::new(0.0, 0.6, 0.3);
        let light_green = RGBColor::new(0.65, 1.0, 0.30);
        let dark_yellow = RGBColor::new(0.61, 0.61, 0.0);
        let light_purple = RGBColor::new(0.65, 0.3, 1.0);
        let dark_purple = RGBColor::new(0.5, 0.0, 1.0);

        // Sphere 1
        let mut sphere = Sphere::new(Point3::new(5.0, 3.0, 0.0), 30.0);
        sphere.set_color_from_rgb_color(yellow);
        world.add_object(Box::new(sphere));

        // Sphere 2
        sphere = Sphere::new(Point3::new(45.0, -7.0, -60.0), 20.0);
        sphere.set_color_from_rgb_color(brown);
        world.add_object(Box::new(sphere));

        // Sphere 3
        let mut sphere = Sphere::new(Point3::new(40.0, 43.0, -100.0), 17.0);
        sphere.set_color_from_rgb_color(dark_green);
        world.add_object(Box::new(sphere));

        // Sphere 4
        sphere = Sphere::new(Point3::new(-20.0, 28.0, -15.0), 20.0);
        sphere.set_color_from_rgb_color(orange);
        world.add_object(Box::new(sphere));

        // Sphere 5
        sphere = Sphere::new(Point3::new(-25.0, -7.0, -35.0), 27.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 6
        let mut sphere = Sphere::new(Point3::new(20.0, -27.0, -35.0), 25.0);
        sphere.set_color_from_rgb_color(light_green);
        world.add_object(Box::new(sphere));

        // Sphere 7
        sphere = Sphere::new(Point3::new(35.0, 18.0, -35.0), 22.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 8
        let mut sphere = Sphere::new(Point3::new(-57.0, -17.0, -50.0), 15.0);
        sphere.set_color_from_rgb_color(brown);
        world.add_object(Box::new(sphere));

        // Sphere 9
        sphere = Sphere::new(Point3::new(-47.0, 16.0, -80.0), 23.0);
        sphere.set_color_from_rgb_color(light_green);
        world.add_object(Box::new(sphere));

        // Sphere 10
        sphere = Sphere::new(Point3::new(-15.0, -32.0, -60.0), 22.0);
        sphere.set_color_from_rgb_color(dark_green);
        world.add_object(Box::new(sphere));

        // Sphere 11
        let mut sphere = Sphere::new(Point3::new(-35.0, -37.0, -80.0), 22.0);
        sphere.set_color_from_rgb_color(dark_yellow);
        world.add_object(Box::new(sphere));

        // Sphere 12
        sphere = Sphere::new(Point3::new(10.0, 43.0, -80.0), 22.0);
        sphere.set_color_from_rgb_color(dark_yellow);
        world.add_object(Box::new(sphere));

        // Sphere 13
        let mut sphere = Sphere::new(Point3::new(30.0, -7.0, -80.0), 10.0);
        sphere.set_color_from_rgb_color(dark_yellow);
        world.add_object(Box::new(sphere));

        // Sphere 14
        sphere = Sphere::new(Point3::new(-40.0, 48.0, -110.0), 18.0);
        sphere.set_color_from_rgb_color(dark_green);
        world.add_object(Box::new(sphere));

        // Sphere 15
        sphere = Sphere::new(Point3::new(-10.0, 53.0, -120.0), 18.0);
        sphere.set_color_from_rgb_color(brown);
        world.add_object(Box::new(sphere));

        // Sphere 16
        let mut sphere = Sphere::new(Point3::new(-55.0, -52.0, -100.0), 10.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 17
        sphere = Sphere::new(Point3::new(5.0, -52.0, -100.0), 15.0);
        sphere.set_color_from_rgb_color(brown);
        world.add_object(Box::new(sphere));

        // Sphere 18
        sphere = Sphere::new(Point3::new(-20.0, -57.0, -120.0), 15.0);
        sphere.set_color_from_rgb_color(dark_purple);
        world.add_object(Box::new(sphere));

        // Sphere 19
        let mut sphere = Sphere::new(Point3::new(55.0, -27.0, -100.0), 17.0);
        sphere.set_color_from_rgb_color(dark_green);
        world.add_object(Box::new(sphere));

        // Sphere 20
        sphere = Sphere::new(Point3::new(50.0, -47.0, -120.0), 15.0);
        sphere.set_color_from_rgb_color(brown);
        world.add_object(Box::new(sphere));

        // Sphere 21
        let mut sphere = Sphere::new(Point3::new(70.0, -42.0, -150.0), 10.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 22
        sphere = Sphere::new(Point3::new(5.0, 73.0, -130.0), 12.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 23
        let mut sphere = Sphere::new(Point3::new(66.0, 21.0, -130.0), 13.0);
        sphere.set_color_from_rgb_color(dark_purple);
        world.add_object(Box::new(sphere));

        // Sphere 24
        sphere = Sphere::new(Point3::new(72.0, -12.0, -140.0), 12.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 25
        sphere = Sphere::new(Point3::new(64.0, 5.0, -160.0), 11.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 26
        let mut sphere = Sphere::new(Point3::new(55.0, 38.0, -160.0), 12.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 27
        sphere = Sphere::new(Point3::new(-73.0, -2.0, -160.0), 12.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 28
        let mut sphere = Sphere::new(Point3::new(30.0, -62.0, -140.0), 15.0);
        sphere.set_color_from_rgb_color(dark_purple);
        world.add_object(Box::new(sphere));

        // Sphere 29
        sphere = Sphere::new(Point3::new(25.0, 63.0, -140.0), 15.0);
        sphere.set_color_from_rgb_color(dark_purple);
        world.add_object(Box::new(sphere));

        // Sphere 30
        sphere = Sphere::new(Point3::new(-60.0, 46.0, -140.0), 15.0);
        sphere.set_color_from_rgb_color(dark_purple);
        world.add_object(Box::new(sphere));

        // Sphere 31
        let mut sphere = Sphere::new(Point3::new(-30.0, 68.0, -130.0), 12.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 32
        sphere = Sphere::new(Point3::new(58.0, 56.0, -180.0), 11.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 33
        let mut sphere = Sphere::new(Point3::new(-63.0, -39.0, -180.0), 11.0);
        sphere.set_color_from_rgb_color(green);
        world.add_object(Box::new(sphere));

        // Sphere 34
        sphere = Sphere::new(Point3::new(46.0, 68.0, -200.0), 10.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));

        // Sphere 35
        sphere = Sphere::new(Point3::new(-3.0, -72.0, -130.0), 12.0);
        sphere.set_color_from_rgb_color(light_purple);
        world.add_object(Box::new(sphere));
    }
}
