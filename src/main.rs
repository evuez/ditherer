extern crate image;

use image::{GenericImageView, ImageBuffer, Pixel, Rgb};
use std::env;
use std::path::Path;

fn main() {
    let mut args = env::args();

    if args.len() < 3 {
        panic!("Wrong number of arguments!");
    }

    args.next();
    dither(&args.next().unwrap(), &args.next().unwrap());
}

fn dither(in_path: &str, out_path: &str) {
    let img = image::open(&Path::new(in_path)).unwrap();
    let (width, height) = img.dimensions();

    let mut img_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let mut errors: Vec<Vec<f32>> = vec![vec![0.0; width as usize]; height as usize];

    for (x, y, pixel) in img.pixels() {
        let (ix, iy) = (x as usize, y as usize);

        let luma = pixel.to_luma()[0] as f32 + errors[iy][ix];
        let discretized: u8 = if luma < 128.0 { 0 } else { 255 };

        img_buffer.put_pixel(x, y, Rgb([discretized; 3]));

        let error: f32 = luma - discretized as f32;

        // Right
        if x + 1 < width {
            errors[iy][ix + 1] += error * 5.0 / 32.0;
        }
        if x + 2 < width {
            errors[iy][ix + 2] += error * 3.0 / 32.0;
        }

        // Bottom
        if y + 1 < height {
            errors[iy + 1][ix] += error * 5.0 / 32.0;
        }
        if y + 2 < height {
            errors[iy + 2][ix] += error * 3.0 / 32.0;
        }

        // Bottom Right
        if y + 1 < height && x + 1 < width {
            errors[iy + 1][ix + 1] += error * 4.0 / 32.0;
        }
        if y + 1 < height && x + 2 < width {
            errors[iy + 1][ix + 2] += error * 2.0 / 32.0;
        }
        if y + 2 < height && x + 1 < width {
            errors[iy + 2][ix + 1] += error * 2.0 / 32.0;
        }

        // Bottom Left
        if y + 1 < height && x > 0 {
            errors[iy + 1][ix - 1] += error * 4.0 / 32.0;
        }
        if y + 1 < height && x > 1 {
            errors[iy + 1][ix - 2] += error * 2.0 / 32.0;
        }
        if y + 2 < height && x > 0 {
            errors[iy + 2][ix - 1] += error * 2.0 / 32.0;
        }
    }

    img_buffer.save(Path::new(out_path)).unwrap();
}
