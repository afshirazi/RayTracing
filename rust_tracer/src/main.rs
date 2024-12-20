use image::{Rgb, RgbImage};
use math::Vec3;
use objects::{Circle, Object, RayOps};

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

fn get_color(ray: &Vec3, origin: &Vec3) -> Vec3 {
    let c = Circle::new(
        Vec3::new(0.0, 0.0, -4.0),
        2.0,
        Vec3::new(0.4, 0.2, 0.76),
        Vec3::empty_vec(),
        10.0,
    );

    c.get_intersect(ray, origin).map_or_else(
        || Vec3::new(0.25, 0.25, 0.25),
        |_| Vec3::new(0.4, 0.2, 0.76),
    )
}

fn main() {
    let mut img = RgbImage::new(1600, 900);
    let eye = Vec3::empty_vec();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let ray = get_ray(&eye, x, y, 1600, 900);
        let px_color = get_color(&ray, &eye);
        *pixel = Rgb([
            (px_color.x * 255.0) as u8,
            (px_color.y * 255.0) as u8,
            (px_color.z * 255.0) as u8,
        ]);
    }

    img.save("test2.png").unwrap();
}
