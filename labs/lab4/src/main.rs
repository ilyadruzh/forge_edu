extern crate image;
extern crate newtonfrac;
extern crate num;
extern crate num_complex;

use image::png::PNGEncoder;
use image::ColorType;
use newtonfrac::newtone_fractal;
use num_complex::Complex;
use std::clone::Clone;
use std::fs::File;
use std::io::Write;
use std::ops::Mul;

static ITER: u32 = 1000;
static MIN: f64 = 1e-6;
static MAX: f64 = 1e+6;
static WIDTH: i32 = 500;
static HEIGHT: i32 = 500;
static X_0: f64 = -0.7;
static X_N: f64 = -1.0;
static Y_0: f64 = -1.0;
static Y_N: f64 = 0.7;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    newtone_fractal::draw(WIDTH, HEIGHT, ITER, MAX, MIN, X_0, X_N, Y_0, Y_N);
}
