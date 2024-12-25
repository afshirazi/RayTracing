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
                Some(_q) => continue 'light_loop,
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
    // let mut z_buf = f64::NEG_INFINITY;
    // let mut obj_idx = None;

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

    let intr_obj = objects
        .iter()
        .map(|obj| (obj, obj.get_intersect(ray, origin)))
        .filter(|(_, intr)| intr.is_some())
        .map(|(obj, intr)| (obj, intr.unwrap().z))
        .max_by(|(_, lz), (_, rz)| lz.total_cmp(rz))
        .map(|(obj, _)| obj);

    if intr_obj.is_none() {
        return Vec3::new(0.3, 0.3, 0.3);
    }

    let intr_obj = intr_obj.unwrap();
    let intr_point = intr_obj.get_intersect(ray, origin).unwrap();
    let normal = intr_obj.get_normal(&intr_point);

    let vis_lights = shadow_rays(&intr_point, intr_obj, objects, lights);

    for light in vis_lights {
        let light_dir = (&light.pos - &intr_point).norm();
        let light_refl = (&(2.0 * (light_dir.dot(&normal)) * &normal) - &light_dir).norm();

        let light_intensity = 250.0 * Vec3::euclid_dist_sq(&light.pos, &intr_point).recip(); // TODO remove hardcode

        let diffuse_term = light_dir.dot(&normal); // doubles to check if light is on correct side of object
        let spec_term = (origin - &intr_point).norm().dot(&light_refl);
        if diffuse_term > 0.0 {
            color_buf.x += intr_obj.get_diff().x * diffuse_term * light.diff.x * light_intensity
                + intr_obj.get_spec().x * spec_term.powf(intr_obj.get_shin()) * light.spec.x * light_intensity;
            color_buf.y += intr_obj.get_diff().y * diffuse_term * light.diff.y * light_intensity
                + intr_obj.get_spec().y * spec_term.powf(intr_obj.get_shin()) * light.spec.y * light_intensity;
            color_buf.z += intr_obj.get_diff().z * diffuse_term * light.diff.z * light_intensity
                + intr_obj.get_spec().z * spec_term.powf(intr_obj.get_shin()) * light.spec.z * light_intensity;
        }
    }

    if depth > 0 && intr_obj.get_spec().dot(intr_obj.get_spec()) > 0.3 {
        let in_ray = origin - &intr_point;
        let refl_ray = (&(2.0 * (in_ray.dot(&normal)) * &normal) - &in_ray).norm();
        color_buf = color_buf + get_color(&refl_ray, &intr_point, objects, lights, Some(intr_obj), depth - 1);
    }

    color_buf
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

    let lights = vec![
        Light::new(
            Vec3::new(2.3, -12.0, -3.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
        Light::new(
            Vec3::new(-1.3, 22.0, 10.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
        ),
    ];

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = get_ray(&eye, x, y, 1600, 900);
        let px_color = get_color(&ray, &eye, &objs, &lights, None, 0);
        *pixel = Rgb([
            (px_color.x.clamp(0.0, 1.0) * 255.0) as u8,
            (px_color.y.clamp(0.0, 1.0) * 255.0) as u8,
            (px_color.z.clamp(0.0, 1.0) * 255.0) as u8,
        ]);
    }

    img.save("test_intensity4.png").unwrap();
}
