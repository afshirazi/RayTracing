use image::{Rgb, RgbImage};
use rand::random;

use crate::{
    light::Light,
    math::Vec3,
    objects::{Object, RayOps},
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

    pub fn render(&self, objects: &Vec<Object>, lights: &Vec<Light>, img: &mut RgbImage) {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut px_color = Vec3::empty_vec();
            for _ in 0..self.samples_per_px {
                let ray = self.get_randomized_ray(x, y, 1600, 900);
                px_color = px_color + Self::get_color(&ray, &self.eye, objects, lights, None, 5);
            }
            px_color = px_color * self.sample_scale;

            // sqrt() for gamma correction (maybe) (idk how it works) 
            let r = (px_color.x.sqrt().clamp(0.0, 1.0) * 255.0);
            let g=(px_color.y.sqrt().clamp(0.0, 1.0) * 255.0);
            let b=(px_color.z.sqrt().clamp(0.0, 1.0) * 255.0);

            *pixel = Rgb([
                r as u8,
                g as u8,
                b as u8,
            ]);
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

    fn shadow_rays<'a>(
        point: &Vec3,
        main_obj: &Object,
        objects: &Vec<Object>,
        lights: &'a Vec<Light>,
    ) -> Vec<&'a Light> {
        let mut vis_lights = vec![];

        'light_loop: for light in lights {
            let light_dir = (&light.pos - point).norm();

            for object in objects {
                if object == main_obj {
                    continue;
                }

                match object.get_intersect(&light_dir, point) {
                    Some(_) => continue 'light_loop,
                    None => (),
                }
            }

            vis_lights.push(light);
        }

        vis_lights
    }

    fn get_color(
        ray: &Vec3,
        origin: &Vec3,
        objects: &Vec<Object>,
        lights: &Vec<Light>,
        src_obj: Option<&Object>,
        depth: u8,
    ) -> Vec3 {
        let mut color_buf = Vec3::new(0.0, 0.0, 0.0);

        let filtered_objects: Vec<&Object> = match src_obj {
            Some(o) => objects.iter().filter(|other_obj| *other_obj != o).collect(),
            None => objects.iter().collect(),
        };

        let intr_obj = filtered_objects
            .iter()
            .map(|obj| (obj, obj.get_intersect(ray, origin)))
            .filter(|(_, intr)| intr.is_some())
            .map(|(obj, intr)| (obj, intr.unwrap()))
            .min_by(|(_, lv), (_, rv)| {
                let ld = Vec3::euclid_dist_sq(lv, origin);
                let rd = Vec3::euclid_dist_sq(rv, origin);
                ld.total_cmp(&rd)
            })
            .map(|(obj, _)| *obj);

        if intr_obj.is_none() {
            return Vec3::new(0.3, 0.3, 0.3);
        }

        let intr_obj = intr_obj.unwrap();
        let intr_point = intr_obj.get_intersect(ray, origin).unwrap();
        let normal = intr_obj.get_normal(&intr_point);

        let vis_lights = Self::shadow_rays(&intr_point, intr_obj, objects, lights);

        for light in vis_lights {
            let light_dir = (&light.pos - &intr_point).norm();
            let light_refl = (&(2.0 * (light_dir.dot(&normal)) * &normal) - &light_dir).norm();

            let light_intensity = 250.0 * Vec3::euclid_dist_sq(&light.pos, &intr_point).recip(); // TODO remove hardcode

            let diffuse_term = light_dir.dot(&normal); // doubles to check if light is on correct side of object
            let spec_term = (origin - &intr_point).norm().dot(&light_refl);
            if diffuse_term > 0.0 {
                color_buf.x +=
                    intr_obj.get_diff().x * diffuse_term * light.diff.x * light_intensity
                        + intr_obj.get_spec().x
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.x
                            * light_intensity;
                color_buf.y +=
                    intr_obj.get_diff().y * diffuse_term * light.diff.y * light_intensity
                        + intr_obj.get_spec().y
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.y
                            * light_intensity;
                color_buf.z +=
                    intr_obj.get_diff().z * diffuse_term * light.diff.z * light_intensity
                        + intr_obj.get_spec().z
                            * spec_term.powf(intr_obj.get_shin())
                            * light.spec.z
                            * light_intensity;
            }
        }

        if depth > 0 && intr_obj.get_spec().dot(intr_obj.get_spec()) > 0.3 {
            let in_ray = origin - &intr_point;
            let refl_ray = (&(2.0 * (in_ray.dot(&normal)) * &normal) - &in_ray).norm();
            color_buf = color_buf
                + Self::get_color(
                    &refl_ray,
                    &intr_point,
                    objects,
                    lights,
                    Some(intr_obj),
                    depth - 1,
                ) * 0.5;
        }

        color_buf
    }
}
