use image::{Rgb, RgbImage};
use math::Vec3;

mod light;
mod math;
mod objects;

fn get_ray(eye: &Vec3, x: u32, y: u32, w: u32, h: u32) -> Vec3 {
    let up = Vec3::new(0.0, 1.0, 0.0);
    let look_at = Vec3::new(0.0, 0.0, -1.0);

    let l = (&look_at - &eye).norm();
    let v = l.cross(&up).norm();
    let u = v.cross(&l);

    let fov_y = 90f64.to_radians();
    let d = 1.0 / (fov_y / 2.0).tan();

    let wh_ratio = w as f64 / h as f64;

    let top_left = &(eye + &(d * &l) - (wh_ratio * &v)) - &u;
    let p = top_left
        + (2.0 * wh_ratio * &v * (x as f64 / w as f64))
        + (2.0 * &u * (y as f64 / h as f64));

    &p - eye
}

fn get_color() -> Vec3 {
    Vec3::new(0.3, 0.3, 0.3)
}

fn main() {
    let mut img = RgbImage::new(1600, 900);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([
            x.rem_euclid(255) as u8,
            y.rem_euclid(255) as u8,
            (x + y).rem_euclid(255) as u8,
        ]);
    }

    img.save("test.png").unwrap();
}
