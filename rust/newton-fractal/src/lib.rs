extern crate cfg_if;
extern crate num;
extern crate wasm_bindgen;
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

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn greet(x: u64) {
//     let s = String::new();
//     let new_str = s.add("other: &str").add(&x.to_string());
//     alert(new_str.as_str());
// }

#[wasm_bindgen]
pub fn greet() {
    alert("Hello");
}


#[wasm_bindgen]
#[derive(Copy, Clone, Debug)]
pub struct Complex {
    x: f64,
    y: f64,
}

#[wasm_bindgen]
pub fn norm_sqr(self_: Complex) -> f64 {
    return 0.0;
}

#[wasm_bindgen]
pub fn mul(self_: Complex, other: Complex) -> Complex {
    Complex {
        x: self_.x * other.x,
        y: self_.y * other.y,
    }
}

#[wasm_bindgen]
pub fn add(self_: Complex, other: Complex) -> Complex {
    Complex {
        x: self_.x + other.x,
        y: self_.y + other.y,
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
pub fn draw(
    mx_input: i32,
    my_input: i32,
    iter: u64,
    max: f64,
    min: f64,
    x_0: f64,
    x_n: f64,
    y_0: f64,
    y_n: f64,
) {
    let mut n: u64 = 0; // итератор
    let mx: i32 = mx_input / 2; // начало экранных координат
    let my: i32 = my_input / 2; // начало экранных координат

    let mut p: f64 = 0.0;

    let mut z = Complex { x: 0.0, y: 0.0 };
    let mut t = Complex { x: 0.0, y: 0.0 };
    let mut d = Complex { x: 0.0, y: 0.0 };

    for y in -my..my {
        for x in -mx..my {
            n = 0;
            z.x = x as f64 * 0.005;
            z.y = y as f64 * 0.005;
            d = z;

            while ((pow(z.x, 2) + pow(z.y, 2)) < max)
                && ((pow(d.x, 2) + pow(d.y, 2)) > min)
                && (n < iter)
            {
                t = z;
                p = pow(pow(t.x, 2) + pow(t.y, 2), 2);

                z.x = (2 as f64 / 3 as f64) * t.x + (pow(t.x, 2) - pow(t.y, 2)) / (3 as f64 * p);
                z.y = (2 as f64 / 3 as f64) * t.y * (1 as f64 - t.x / p);
                d.x = t.x.abs() - z.x.abs();
                d.y = t.y.abs() - z.y.abs();
                n = n + 1;
            }

            // Выбираем цвет - pen.Color = Color.FromArgb(255, (n*9) % 255, 0, (n*9) % 255);
            // Рисуем прямоугольник - g.DrawRectangle(pen, mx + x, my + y, 1, 1);
            //draw_newtone_fractal(mx + x, my + y);
        }
    }
}
