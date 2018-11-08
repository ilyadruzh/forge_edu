extern crate image;
extern crate newtonfrac;
extern crate num;
extern crate num_complex;

use newtonfrac::newtone_fractal;
use num_complex::Complex;
use std::io::Write;
use std::ops::Mul;

static ITER: u64 = 500;
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

    //newtone_fractal::create_img(WIDTH, HEIGHT, ITER, MAX, MIN, X_0, X_N, Y_0, Y_N);
    newtone_fractal::draw(WIDTH, HEIGHT, ITER, MAX, MIN, X_0, X_N, Y_0, Y_N);

    // if args.len() != 5 {
    //     writeln!(
    //         std::io::stderr(),
    //         "Порядок вызова: newton-fractal FILE PIXELS UPPERLEFT LOWERRIGHT"
    //     ).unwrap();
    //     writeln!(
    //         std::io::stderr(),
    //         "Пример: mandel.png 1000x750 -1.20,0.35 -1,0.20"
    //     ).unwrap();
    //     std::process::exit(1);
    // }

    // let bounds = newtone_fractal::parse_display_size(&args[2], 'x')
    //     .expect("ошибка при разборе размеров изображения");
    // let upper_left = newtone_fractal::parse_complex(&args[3]).expect("ошибка при разборе координат левого верхнего угла");
    // let lower_right = newtone_fractal::parse_complex(&args[4]).expect("ошибка при разборе координат правого нижнего угла");

    // let mut pixels = vec![0; bounds.0 * bounds.1];

    // newtone_fractal::render(&mut pixels, bounds, upper_left, lower_right, ITER);
    // newtone_fractal::write_image(&args[1], &pixels, bounds)
    //     .expect("ошибка при записи PNG-файла");
}
