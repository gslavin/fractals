#[macro_use]
extern crate bmp;
extern crate num;

use num::complex::*;
use bmp::{Image, Pixel};

const SIZE: u32 = 256;
const ESCAPE_LIMIT: f64 = 1000000.0;
const ITER_LIMIT: u32 = 200;
const WIDTH: f64 = 0.005;
const HEIGHT: f64 = 0.010;
const X_CENTER: f64 = 1.941;
const Y_CENTER: f64 = 0.004;

// Burning ship
#[inline]
fn burning_ship() -> Box<Fn(Complex64, Complex64) -> Complex64> 
{
    return Box::new(move |z: Complex64, c: Complex64| (Complex{re: z.re.abs(), im: z.im.abs()}).powf(2.0) + c);
}

// Mendelbrot 
fn mandelbrot() -> Box<Fn(Complex64, Complex64) -> Complex64> 
{
    return Box::new(move |z: Complex64, c: Complex64| z.powf(2.0) + c);
}

fn run_until_escape(gen_fn: Box<Fn(Complex64, Complex64) -> Complex64>,
    c: Complex64, escape_limit: f64, iter_limit: u32) -> u32
{
    let mut z: Complex64 = Complex64{ re: 0.0, im: 0.0};
    let mut n: u32 = 0; 
    while z.norm_sqr() < escape_limit && n < iter_limit {
        n += 1;
        z = (*gen_fn)(z, c);
    }

    return n;
}

fn main() {
    let mut pixels = vec![0; (SIZE*SIZE) as usize];

    let coordinates: Vec<(u32, u32)> = (0..SIZE).flat_map(move |x| (0..SIZE).map(move |y| (x,y))).collect();
    for &(x, y) in &coordinates {
        let float_x: f64 = WIDTH*((x as f64)/(SIZE as f64) - 0.5) - X_CENTER;
        let float_y: f64 = HEIGHT*((y as f64)/(SIZE as f64) - 0.5) - Y_CENTER;
        let iterations = run_until_escape(burning_ship(), Complex{re: float_x, im: float_y}, 
                                          ESCAPE_LIMIT, ITER_LIMIT);
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
