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
    let mut img = Image::new(SIZE, SIZE);
    /*
    // Fixed size Array, using vectors instead
    let mut pixels: [[u32; (SIZE as usize)]; (SIZE as usize)] =
        [[0; (SIZE as usize)]; (SIZE as usize)];
    */
    let mut pixels = vec![vec![0; (SIZE as usize)]; (SIZE as usize)];

    for (x,y) in img.coordinates()
    {
        let float_x: f64 = WIDTH*((x as f64)/(SIZE as f64) - 0.5);
        let float_y: f64 = HEIGHT*((y as f64)/(SIZE as f64) - 0.5);
        let iterations = run_until_escape(Complex{re:float_x, im:float_y}, 
            ESCAPE_LIMIT, ITER_LIMIT,
            fractal_func(C_VAL));
        pixels[x as usize][y as usize] = iterations;
    }

    let mut max = 0;
    for x in 0..SIZE {
        for y in 0..SIZE {
            let current = pixels[x as usize][y as usize];
            if current < ITER_LIMIT && current > max {
               max = current; 
            }
        }
    }
    println!("{}", max);

    for x in 0..SIZE {
        for y in 0..SIZE {
            let current = pixels[x as usize][y as usize];
            if current < ITER_LIMIT {
                pixels[x as usize][y as usize] *= 256;
                pixels[x as usize][y as usize] /= max;
            }
            else {
                pixels[x as usize][y as usize] = 256;
            }
        }
    }

    for (x,y) in img.coordinates()
    {
        let val = pixels[x as usize][y as usize];
        img.set_pixel(x, y, px!(val, val, val));
    }
    let _ = img.save("test.bmp");
}
