use image::{Rgb, RgbImage};


fn main() {
    let img_buf = Vec::new();
    let mut img = RgbImage::from_raw(1600, 900, img_buf).unwrap();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x.rem_euclid(255) as u8, y.rem_euclid(255) as u8, (x+y).rem_euclid(255) as u8]);
    }

    img.save("test.png").unwrap();
}

