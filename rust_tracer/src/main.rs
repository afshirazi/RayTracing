use image::{Rgb, RgbImage};


fn main() {
    let mut img = RgbImage::new(1600, 900);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        *pixel = Rgb([x.rem_euclid(255) as u8, y.rem_euclid(255) as u8, (x+y).rem_euclid(255) as u8]);
    }

    img.save("test.png").unwrap();
}

