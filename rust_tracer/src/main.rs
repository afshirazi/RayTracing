use core::f64;

use image::{Rgb, RgbImage};
use light::Light;
use math::Vec3;
use objects::{Circle, Object, RayOps, Triangle};

mod light;
mod math;
mod objects;

fn get_ray(eye: &Vec3, x: u32, y: u32, w: u32, h: u32) -> Vec3 {
    //TODO clean up and decide where variables should be
    let up = Vec3::new(0.0, 1.0, 0.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);

    let l = (&look_at - &eye).norm();
    let v = l.cross(&up).norm();
    let u = v.cross(&l);

    // let fov_y = 90f64.to_radians();
    // let d = 1.0 / (fov_y / 2.0).tan();
    let d = 1.2;

    let wh_ratio = w as f64 / h as f64;

    let top_left = &(eye + &(d * &l) - (wh_ratio * &v)) - &u;
    let p = top_left
        + (2.0 * wh_ratio * &v * (x as f64 / w as f64))
        + (2.0 * &u * (y as f64 / h as f64));

    (&p - eye).norm()
}

fn get_color(ray: &Vec3, origin: &Vec3, objects: &Vec<Object>) -> Vec3 {
    // let mut color_buf = Vec3::new(0.3, 0.3, 0.3);
    // let mut z_buf = f64::NEG_INFINITY;
    // let mut obj_idx = None;

    let light = Light::default_light();

    // for (idx, obj) in objects.iter().enumerate() {
    //     let intersect = match obj.get_intersect(ray, origin) {
    //         Some(point) => point,
    //         None => continue,
    //     };

    //     if intersect.z > z_buf {
    //         z_buf = intersect.z;
    //         obj_idx = Some(idx);
    //     }
    // }
    let obj_opt = objects
        .iter()
        .map(|obj| (obj, obj.get_intersect(ray, origin)))
        .filter(|(_, res)| res.is_some())
        .map(|(o, res)| (o, res.unwrap().z))
        .max_by(|x, y| x.1.total_cmp(&y.1))
        .map(|(o, _)| o);
    // if let Some(idx) = obj_idx {
    //     let obj = objects.get(idx).unwrap();
    //     let intr_point = obj.get_intersect(ray, origin).unwrap();
    //     let normal = obj.get_normal(&intr_point);

    //     let light_dir = (&light.pos - &intr_point).norm();
    //     let light_refl = (&(2.0 * (light_dir.dot(&normal)) * &normal) - &light_dir).norm();

    //     let diffuse_term = light_dir.dot(&normal); // doubles to check if light is on correct side of object
    //     let spec_term = (origin - &intr_point).norm().dot(&light_refl);
    //     if diffuse_term > 0.0 {
    //         let r = obj.get_diff().x * diffuse_term * light.diff.x
    //             + obj.get_spec().x * spec_term.powf(obj.get_shin()) * light.spec.x;
    //         let g = obj.get_diff().y * diffuse_term * light.diff.y
    //             + obj.get_spec().y * spec_term.powf(obj.get_shin()) * light.spec.y;
    //         let b = obj.get_diff().z * diffuse_term * light.diff.z
    //             + obj.get_spec().z * spec_term.powf(obj.get_shin()) * light.spec.z;

    //         color_buf = Vec3::new(r, g, b);
    //     }
    // }
    if let Some(obj) = obj_opt {
        let intr_point = obj.get_intersect(ray, origin).unwrap();
        let normal = obj.get_normal(&intr_point);

        let light_dir = (&light.pos - &intr_point).norm();
        let light_refl = (&(2.0 * (light_dir.dot(&normal)) * &normal) - &light_dir).norm();

        let diffuse_term = light_dir.dot(&normal); // doubles to check if light is on correct side of object
        let spec_term = (origin - &intr_point).norm().dot(&light_refl);
        if diffuse_term > 0.0 {
            let r = obj.get_diff().x * diffuse_term * light.diff.x
                + obj.get_spec().x * spec_term.powf(obj.get_shin()) * light.spec.x;
            let g = obj.get_diff().y * diffuse_term * light.diff.y
                + obj.get_spec().y * spec_term.powf(obj.get_shin()) * light.spec.y;
            let b = obj.get_diff().z * diffuse_term * light.diff.z
                + obj.get_spec().z * spec_term.powf(obj.get_shin()) * light.spec.z;

            // color_buf = Vec3::new(r, g, b);
            return Vec3::new(r, g, b);
        }
    }

    // color_buf
    Vec3::new(0.3, 0.3, 0.3)
}

fn main() {
    let mut img = RgbImage::new(1600, 900);
    let eye = Vec3::empty_vec();

    let objs = vec![
        Object::Circle(Circle::new(
            Vec3::new(2.0, -3.0, -10.0),
            2.0,
            Vec3::new(0.4, 0.2, 0.76),
            Vec3::new(0.4, 0.2, 0.76),
            10.0,
        )),
        Object::Circle(Circle::from_color(
            Vec3::new(-2.0, 3.0, -10.0),
            2.0,
            &Vec3::new(0.4, 0.2, 0.76),
        )),
        Object::Triangle(Triangle::from_color(
            Vec3::new(-4.5, -3.0, -9.0),
            Vec3::new(2.0, -3.0, -10.0),
            Vec3::new(-3.5, 1.5, -11.0),
            &Vec3::new(0.4, 0.2, 0.76),
        )),
    ];

    // let lights = vec![Light::new(Vec3::new(1.3, -22.0, 10.0), Vec3::new(1.0, 1.0, 1.0), Vec3::new(1.0, 1.0, 1.0))];

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = get_ray(&eye, x, y, 1600, 900);
        let px_color = get_color(&ray, &eye, &objs);
        *pixel = Rgb([
            (px_color.x * 255.0) as u8,
            (px_color.y * 255.0) as u8,
            (px_color.z * 255.0) as u8,
        ]);
    }

    img.save("test6.png").unwrap();
}
