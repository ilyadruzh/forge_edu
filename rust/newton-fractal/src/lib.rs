extern crate cfg_if;
extern crate image;
extern crate num;
extern crate wasm_bindgen;
extern crate web_sys;

use cfg_if::cfg_if;
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;
use wasm_bindgen::prelude::*;

mod utils;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(x: u32) {
    alert("x");
}

////////////////// COMPLEX /////////////////////////////

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Complex {
    re: f64,
    im: f64,
}

#[wasm_bindgen]
pub fn sqr(self_: Complex) -> Complex {
    Complex {
        re: (self_.re * self_.re) - (self_.im * self_.im),
        im: (2 as f64 * self_.re * self_.im),
    }
}

// Как происходит умножение комплексных чисел
#[wasm_bindgen]
pub fn mul(self_: Complex, other: Complex) -> Complex {
    Complex {
        re: (self_.re * other.re) - (self_.im * other.im),
        im: (self_.re * other.im) + (self_.im * other.re),
    }
}

#[wasm_bindgen]
pub fn add(self_: Complex, other: Complex) -> Complex {
    Complex {
        re: self_.re + other.re,
        im: self_.im + other.im,
    }
}

// Как происходит вычитание комплексных чисел
#[wasm_bindgen]
pub fn sub(self_: Complex, other: Complex) -> Complex {
    // Complex {
    //     re: ((self_.re * other.re) + (self_.im * other.im))
    //         / ((other.re * other.re) + (other.im * other.im)),
    //     im: (-(self_.re * other.im) + (self_.im * other.re))
    //         / (other.re * other.re + other.im * other.im),
    // }

    Complex {
        re: self_.re - other.re,
        im: self_.im - other.im,
    }
}

#[wasm_bindgen]
pub fn abs(self_: Complex) -> f64 {
    f64::sqrt((self_.re * self_.re) + (self_.im * self_.im))
}

#[wasm_bindgen]
pub fn arg(z: Complex) -> f64 {
    z.im.atan2(z.re)
}

#[wasm_bindgen]
pub fn pow(z: Complex, n: i32) -> Complex {
    let i: i32 = 0;
    let mut z1: Complex = Complex { re: 1.0, im: 0.0 };
    for x in i..n {
        z1 = mul(z, z1);
    }
    return z1;
}

///////////////////////////// NEWTOM /////////////////////////////

// pub fn func(z: Complex) -> Complex {
//     return mul(z, mul(z, z)) - 1;
// }

// pub fn dfunc(z: Complex) -> Complex {
//     return mul(z, mul(z, 3));
// }

#[allow(dead_code)]
#[wasm_bindgen]
pub fn draw(
    mx_input: i32,
    my_input: i32,
    iter: u32,
    max: f64,
    min: f64,
    x_0: f64,
    x_n: f64,
    y_0: f64,
    y_n: f64,
) {
    let mut n: u32 = 0; // итератор
    let mx: i32 = mx_input / 2; // начало экранных координат
    let my: i32 = my_input / 2; // начало экранных координат

    let mut p: f64 = 0.0;

    let mut z = Complex { re: 0.0, im: 0.0 }; // Основаная функция
    let mut t = Complex { re: 0.0, im: 0.0 };
    let mut d = Complex { re: 0.0, im: 0.0 }; // Производная функция

    for y in -my..my {
        for x in -mx..my {
            n = 0;

            // z.re = x * 4.0 as f64 / 675 - 2; // Что это за формула??
            // z.im = -(y * 4.0 as f64 / 675 + 2);

            d = z;

            // Условия, при которых комплексное число устремляется в бесконечность
            // Первый параметр - функция от Z - функции комплексного числа
            // Второй параметр функция от D - производная от функции
            // Третий параметр N - счетчик
            // while (func(z) < max && dfunc(z) > min && n < iter) {
            // n = n + 1;
            // }

            // while ((pow(z, 2) < max) && (pow(d, 2) > min) && (n < iter)) {
            //    t = z; // временная переменная

            // вычисляем комплексную функцию
            //      z.x =

            // вычисляем производную комплексной функции
            //    }

            create_image(x, y, choose_color(x, y, n));

            //     t = z;

            //     p = pow(pow(t.re, 2) + pow(t.im, 2), 2); // для чего это?

            //     z.re = (2.0 / 3.0) * t.re + (pow(t.re, 2) - pow(t.im, 2)) / (3.0 * p);
            //     z.im = (2.0 / 3.0) * t.im * (1.0 - t.re / p);

            //     d.re = t.re.abs() - z.re.abs();
            //     d.im = t.im.abs() - z.im.abs();
        }
    }
}

///////////////////////////// CREATE IMG /////////////////////////////
#[wasm_bindgen]
pub fn choose_color(x: i32, y: i32, n: u32) -> (u32, u32, u32) {
    // В зависимисомти, от X, Y, N выбирается цвет
    (0, 0, 0)
}
#[wasm_bindgen]
pub fn create_image(x: i32, y: i32, color: (u32, u32, u32)) {}

fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex,
    lower_right: Complex,
) -> Complex {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex {
        re: upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        im: upper_left.im - pixel.1 as f64 * height / bounds.1 as f64, // Почему здесь вычитание? pixel.1 увеличивается при движении вниз,
                                                                       // тогда как мнимая часть увеличивается при движении вверх.
    }
}

fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex,
    lower_right: Complex,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);
    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize),
) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(
        &pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        ColorType::Gray(8),
    )?;
    Ok(())
}
