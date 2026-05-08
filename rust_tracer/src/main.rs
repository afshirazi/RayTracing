use camera::Camera;
use image::RgbImage;
use light::PointLight;
use math::Vec3;
use objects::{Circle, Object, Triangle};

use crate::{
    bxdf::{
        Bxdfs, conductor_bxdf::ConductorBxdf, dielectric_bxdf::DielectricBxdf,
        trowbridge_reitz_distribution::TrowbridgeReitzDistribution,
    },
    spectrum::{piecewise_linear_spectrum::PiecewiseLinearSpectrum, sampled_spectrum::{SampledSpectrum, SampledWavelengths}},
};

mod bxdf;
mod camera;
mod integrator;
mod light;
mod math;
mod objects;
mod sampler;
mod spectrum;

fn main() {
    let mut img = RgbImage::new(1600, 900);

    let dielectric = DielectricBxdf::new(TrowbridgeReitzDistribution::zero(), 1.6);
    let conductor = ConductorBxdf::new(
        TrowbridgeReitzDistribution::zero(),
        SampledSpectrum::new([0.1, 0.3, 1.4, 0.0]),
        SampledSpectrum::new([3.1, 2.8, 2.0, 0.0]),
    );
    let lambdas = SampledWavelengths::sample_uniform(rand::random(), None, None);
    let piecewise_spec = PiecewiseLinearSpectrum::new(vec![400., 500., 600., 700.], vec![0.85, 0.43, 0.16, 0.52]);

    let objects = vec![
        Object::Circle(Circle::from_color(
            Vec3::new(2.0, -4.0, -10.0),
            2.0,
            Bxdfs::Dielectric(dielectric),
        )),
        Object::Circle(Circle::from_color(
            Vec3::new(0.0, 5.0, -7.0),
            4.0,
            Bxdfs::Conductor(conductor),
        )),
        Object::Triangle(Triangle::from_color(
            Vec3::new(-4.5, -3.0, -9.0),
            Vec3::new(2.0, -3.0, -10.0),
            Vec3::new(-3.5, 1.5, -11.0),
            SampledSpectrum::new([0.4, 0.2, 0.76, 0.0]),
        )),
    ];

    let lights = vec![
        PointLight::new(Vec3::new(2.3, -12.0, -3.0), 1.0, &piecewise_spec),
        PointLight::new(Vec3::new(-1.3, 22.0, 10.0), 1.0, &piecewise_spec),
    ];

    let camera = Camera::new(
        Vec3::empty_vec(),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        90_f64.to_radians(),
        1,
    );

    camera.render(&objects, &lights, &lambdas, &mut img);

    img.save("test_f32.png").unwrap();
}
