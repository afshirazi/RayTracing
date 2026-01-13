use image::{Rgb, RgbImage};
use rand::random;

use crate::{
    integrator::{Integrator, simple_path_integrator::SimplePathIntegrator}, light::PointLight, math::Vec3, objects::Object, sampler::Sampler
};

pub struct Camera {
    eye: Vec3,
    up: Vec3,
    look_at: Vec3,
    d: f64,
    samples_per_px: u8,
    sample_scale: f64,
}

impl Camera {
    pub fn new(eye: Vec3, up: Vec3, look_at: Vec3, fov_y: f64, samples_per_px: u8) -> Self {
        Camera {
            eye,
            up,
            look_at,
            d: 1.0 / (fov_y / 2.0).tan(),
            samples_per_px,
            sample_scale: (samples_per_px as f64).recip(),
        }
    }

    pub fn render(&self, objects: &[Object], lights: &[PointLight], img: &mut RgbImage) {
        let li = SimplePathIntegrator::incident_radiance;
        struct TempSampler;
        impl Sampler for TempSampler {}
        let temp_sampler = TempSampler;
        // ^^^^^^ This is all temporary garbage TODO: remove

        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut px_color = Vec3::empty_vec();
            for _ in 0..self.samples_per_px {
                let ray = self.get_randomized_ray(x, y, 1600, 900);
                px_color +=  li(&ray, &self.eye, &temp_sampler, objects, lights, None, 5);
            }
            px_color = px_color * self.sample_scale;

            // sqrt() for gamma correction (maybe) (idk how it works)
            let r = px_color.x.sqrt().clamp(0.0, 1.0) * 255.0;
            let g = px_color.y.sqrt().clamp(0.0, 1.0) * 255.0;
            let b = px_color.z.sqrt().clamp(0.0, 1.0) * 255.0;

            *pixel = Rgb([r as u8, g as u8, b as u8]);
        }
    }

    fn get_randomized_ray(&self, x: u32, y: u32, w: u32, h: u32) -> Vec3 {
        let l = (&self.look_at - &self.eye).norm();
        let v = l.cross(&self.up).norm();
        let u = v.cross(&l);

        let wh_ratio = w as f64 / h as f64;

        let offset_x = (random::<f64>() - 0.5 + x as f64) / w as f64;
        let offset_y = (random::<f64>() - 0.5 + y as f64) / h as f64;

        let top_left = &(&self.eye + &(self.d * &l) - (wh_ratio * &v)) - &u;
        let p = top_left + (2.0 * wh_ratio * &v * offset_x) + (2.0 * &u * offset_y);

        (&p - &self.eye).norm()
    }

    // fn get_color(
    //     ray: &Vec3,
    //     origin: &Vec3,
    //     objects: &Vec<Object>,
    //     lights: &Vec<Light>,
    //     src_obj: Option<&Object>,
    //     depth: u8,
    // ) -> Vec3 {

    // }
}
