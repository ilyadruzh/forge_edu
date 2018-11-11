extern crate cfg_if;
extern crate num;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;

use std::ops::Add;

mod utils;

use cfg_if::cfg_if;
use num::pow;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn using_a_macro() {
    console_log!("Hello {}!", "world");
    console_log!("Let's print some numbers...");
    console_log!("1 + 3 = {}", 1 + 3);
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn greet(x: u32) {
    log_u32(x);
}

#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Complex {
    re: f64, // действительное число
    im: f64, // мнимая единица
}

#[wasm_bindgen]
pub fn sqr(self_: Complex) -> Complex {
    Complex {
        re: (self_.re * self_.re) - (self_.im * self_.im),
        im: (2 as f64 * self_.re * self_.im),
    }
}

#[wasm_bindgen]
pub fn mul(self_: Complex, other: Complex) -> Complex {
    Complex {
        re: self_.re * other.re,
        im: self_.im * other.im,
    }
}

#[wasm_bindgen]
pub fn add(self_: Complex, other: Complex) -> Complex {
    Complex {
        re: self_.re + other.re,
        im: self_.im + other.im,
    }
}

#[wasm_bindgen]
pub fn abs(self_: Complex) -> f64 {
    f64::sqrt((self_.re * self_.re) + (self_.im * self_.im))
}

pub fn color(n: u32) -> (u32, u32, u32) {
    (0, 0, 0)
}

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

    let mut z = Complex { re: 0.0, im: 0.0 };
    let mut t = Complex { re: 0.0, im: 0.0 };
    let mut d = Complex { re: 0.0, im: 0.0 };

    for y in -my..my {
        for x in -mx..my {
            n = 0;
            z.re = x as f64 * 0.005;
            z.im = y as f64 * 0.005;
            d = z;

            while ((pow(z.re, 2) + pow(z.im, 2)) < max)
                && ((pow(d.re, 2) + pow(d.im, 2)) > min)
                && (n < iter)
            {
                t = z;
                p = pow(pow(t.re, 2) + pow(t.im, 2), 2);

                z.re =
                    (2 as f64 / 3 as f64) * t.re + (pow(t.re, 2) - pow(t.im, 2)) / (3 as f64 * p);
                z.im = (2 as f64 / 3 as f64) * t.im * (1 as f64 - t.re / p);
                d.re = t.re.abs() - z.re.abs();
                d.im = t.im.abs() - z.im.abs();
                n = n + 1;
            }

            // Выбираем цвет - в зависимости от n - pen.Color = Color.FromArgb(255, (n*9) % 255, 0, (n*9) % 255);
            // Рисуем прямоугольник - передаем цвет и точку х и у
            // draw_newtone_fractal(mx + x, my + y);
        }
    }
}

pub fn create_image(x: i32, y: i32, color: (u32, u32, u32)) {
    
}
