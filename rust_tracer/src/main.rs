use camera::Camera;
use image::RgbImage;
use light::PointLight;
use math::Vec3;
use objects::{Circle, Object, Triangle};

mod camera;
mod light;
mod math;
mod objects;
mod integrator;
mod sampler;
mod bxdf;

fn main() {
    let mut img = RgbImage::new(1600, 900);

    let objects = vec![
        Object::Circle(Circle::from_color(
            Vec3::new(2.0, -4.0, -10.0),
            2.0,
            Vec3::new(0.4, 0.2, 0.76),
        )),
        Object::Circle(Circle::from_color(
            Vec3::new(-2.0, 105.0, -10.0),
            100.0,
            Vec3::new(0.17, 0.6, 0.23),
        )),
        Object::Triangle(Triangle::from_color(
            Vec3::new(-4.5, -3.0, -9.0),
            Vec3::new(2.0, -3.0, -10.0),
            Vec3::new(-3.5, 1.5, -11.0),
            &Vec3::new(0.4, 0.2, 0.76),
        )),
    ];

    let lights = vec![
        PointLight::new(
            Vec3::new(2.3, -12.0, -3.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
        PointLight::new(
            Vec3::new(-1.3, 22.0, 10.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
    ];

    let camera = Camera::new(
        Vec3::empty_vec(),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        90_f64.to_radians(),
        20,
    );

    camera.render(&objects, &lights, &mut img);

    img.save("after_bxdf.png").unwrap();
}
