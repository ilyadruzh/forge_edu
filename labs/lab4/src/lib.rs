extern crate image;
extern crate num;
extern crate num_complex;

pub mod newtone_fractal {
    use image;
    use image::png::PNGEncoder;
    use image::{ColorType, GrayImage, ImageBuffer, Luma, Pixel};
    use num_complex::Complex;
    use std::clone::Clone;
    use std::f64;
    use std::fs::File;
    use std::io::{Error, Write};
    use std::ops::{Add, Mul, Sub};
    use std::str::FromStr;

    pub fn abs(z: Complex<f64>) -> f64 {
        ((z.re * z.re) + (z.im * z.im)).sqrt()
    }

    pub fn choose_color(x: i32, y: i32, n: u32) -> (u32, u32, u32) {
        // В зависимисомти, от X, Y, N выбирается цвет
        (0, 0, 0)
    }

    pub fn create_image(x: i32, y: i32, color: (u32, u32, u32)) {}

    pub fn zfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z * z).sub(1 as f64)
    }

    pub fn dfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z).scale(3 as f64)
    }

    #[allow(dead_code)]
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
        let mut tol = 0.00001;

        let mut z = Complex { re: 0.0, im: 0.0 }; // Основаная функция
        let mut t = Complex { re: 0.0, im: 0.0 };
        let mut d = Complex { re: 0.0, im: 0.0 }; // Производная функция

        let bounds = (mx_input, my_input);
        let mut pixels = vec![0; bounds.0 as usize * bounds.1 as usize];

        let mut imgbuf = image::GrayImage::new(mx_input as u32, my_input as u32);
        let scalex = 4.0 / mx_input as f32;
        let scaley = 4.0 / my_input as f32;

        for y in 0..my {
            for x in 0..my {
                let mut zt: Complex<f64> = Complex {
                    re: x as f64 * 4.0 / mx as f64 - 2 as f64,
                    im: -y as f64 * 4.0 / my as f64 + 2 as f64,
                };

                for i in 0..iter {
                    if abs(zfunc(zt)) < tol {
                        break;
                    }
                    zt = zt - zfunc(zt) / dfunc(z);
                }

                imgbuf.put_pixel(x as u32, y as u32, image::Luma([n as u8 + 3 as u8]));

                // n = 0;
                // d = z;

                // Условия, при которых комплексное число устремляется в бесконечность
                // Первый параметр - функция от Z - функции комплексного числа
                // Второй параметр функция от D - производная от функции
                // Третий параметр N - счетчик

                // let mut c: Complex<f64> = Complex {
                //     re: mx as f64,
                //     im: my as f64,
                // };

                // while (abs(z) < max && n < iter) {
                //     z = z.mul(z).add(c);
                //     n = n + 1;
                // }

                // while ((pow(z, 2) < max) && (pow(d, 2) > min) && (n < iter)) {
                //    t = z; // временная переменная
                // вычисляем комплексную функцию
                //      z.x =
                // вычисляем производную комплексной функции
                //     p = pow(pow(t.re, 2) + pow(t.im, 2), 2); // для чего это?
                //     z.re = (2.0 / 3.0) * t.re + (pow(t.re, 2) - pow(t.im, 2)) / (3.0 * p);
                //     z.im = (2.0 / 3.0) * t.im * (1.0 - t.re / p);
                //     d.re = t.re.abs() - z.re.abs();
                //     d.im = t.im.abs() - z.im.abs();

                // imgbuf.put_pixel(x as u32, y as u32, image::Luma([n as u8]));
            }
        }

        imgbuf.save("fractal.png").unwrap();
    }
}
