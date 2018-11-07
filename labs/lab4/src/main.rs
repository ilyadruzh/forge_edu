extern crate image;
extern crate newtonfrac;
extern crate num;
extern crate num_complex;

use newtonfrac::newtone_fractal;
use num_complex::Complex;
use std::io::Write;
use std::ops::Mul;

//mod newtone_fractal;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let x_0 = -0.7;
    let x_n = 1.0;
    let y_0 = -1.0;
    let y_n = 0.7;

    newtone_fractal::create_img(500, 500, 1000, x_0, x_n, y_0, y_n);

    if args.len() != 5 {
        writeln!(
            std::io::stderr(),
            "Порядок вызова: newton-fractal FILE PIXELS UPPERLEFT LOWERRIGHT"
        ).unwrap();
        writeln!(
            std::io::stderr(),
            "Пример: mandel.png 1000x750 -1.20,0.35 -1,0.20"
        ).unwrap();
        std::process::exit(1);
    }

    let bounds = newtone_fractal::parse_display_size(&args[2], 'x')
        .expect("ошибка при разборе размеров изображения");
    let upper_left = newtone_fractal::parse_complex(&args[3]).expect("ошибка при разборе координат левого верхнего угла");
    let lower_right = newtone_fractal::parse_complex(&args[4]).expect("ошибка при разборе координат правого нижнего угла");

    let mut pixels = vec![0; bounds.0 * bounds.1];

    newtone_fractal::render(
        &mut pixels,
        bounds,
        upper_left,
        lower_right,
        x_0,
        x_n,
        y_0,
        y_n,
    );
    newtone_fractal::write_image(&args[1], &pixels, bounds)
        .expect("ошибка при записи PNG-файла");
}
