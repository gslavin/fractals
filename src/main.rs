#[macro_use]
extern crate bmp;
extern crate num;

use num::complex::*;
use bmp::{Image, Pixel};

const SIZE: u32 = 512;
const ESCAPE_LIMIT: f64 = 1000000.0;
const ITER_LIMIT: u32 = 200;
const WIDTH: f64 = 3.0;
const HEIGHT: f64 = 3.0;
const C_VAL: Complex64 = Complex{re: 0.25, im: 0.5};

// Mendelbrot transform
fn fractal_func(c: Complex64) -> Box<Fn(Complex64) -> Complex64> 
{
    return Box::new(move |z: Complex64| z.powf(2.0) + c);
}

fn run_until_escape(z: Complex64, escape_limit: f64, iter_limit: u32,
                    gen_fn: Box<Fn(Complex64) -> Complex64>) -> u32
{
    let mut temp = z;
    let mut n: u32 = 0; 
    while temp.norm_sqr() < escape_limit && n < iter_limit {
        n += 1;
        temp = (*gen_fn)(temp);
    }

    return n;
}

fn main() {
    let mut pixels = vec![0; (SIZE*SIZE) as usize];

    let coordinates: Vec<(u32, u32)> = (0..SIZE).flat_map(move |x| (0..SIZE).map(move |y| (x,y))).collect();
    for &(x, y) in &coordinates {
        let float_x: f64 = WIDTH*((x as f64)/(SIZE as f64) - 0.5);
        let float_y: f64 = HEIGHT*((y as f64)/(SIZE as f64) - 0.5);
        let iterations = run_until_escape(Complex{re:float_x, im:float_y}, 
                                          ESCAPE_LIMIT, ITER_LIMIT,
                                          fractal_func(C_VAL));
        pixels[(SIZE*x + y) as usize] = iterations;
    }

    let max = pixels.iter().cloned().filter(|&x| x < ITER_LIMIT).max().unwrap();
    println!("max pixel: {}", max);

    for &(x, y) in &coordinates {
            let current = pixels[(SIZE*x + y) as usize];
            if current < ITER_LIMIT {
                pixels[(SIZE*x + y) as usize] *= 256;
                pixels[(SIZE*x + y) as usize] /= max;
            }
            else {
                pixels[(SIZE*x + y) as usize] = 256;
            }
    }

    let mut img = Image::new(SIZE, SIZE);
    for (x,y) in img.coordinates()
    {
        let val = pixels[(SIZE*x + y) as usize];
        img.set_pixel(x, y, px!(val, val, val));
    }

    let _ = img.save("test.bmp");
}
