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
    use std::ops::{Add, Div, Mul, Sub};
    use std::str::FromStr;

    pub fn abs(z: Complex<f64>) -> f64 {
        ((z.re * z.re) + (z.im * z.im)).sqrt()
    }

    pub fn choose_color(x: i32, y: i32, n: i32) -> [u8; 3] {
        [0, 0, 0]
    }

    // F(x) = x^numRoots - 1
    // Возводим в 3 степень. TODO: улучшить, чтобы степень приходила параметром
    pub fn zfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z * z).sub(1 as f64)
    }

    // dF(x) = numRoots*x^(numRoots -1)
    // TODO: улучшить, чтобы степень приходила параметром
    pub fn dfunc(z: Complex<f64>) -> Complex<f64> {
        (z * z).scale(3 as f64)
    }

    pub fn arg(z: Complex<f64>) -> f64 {
        z.im.atan2(z.re)
    }

    #[allow(dead_code)]
    pub fn draw(mx_input: i32, my_input: i32, iter: i32, x_0: f64, x_n: f64, y_0: f64, y_n: f64) {
        let tolerance = 0.00001; // Work until the epsilon squared < this.

        let r1 = Complex { re: 1.0, im: 0.0 };
        let r2 = Complex { re: x_0, im: y_0 };
        let r3 = Complex { re: x_n, im: y_n };

        let mut imgbuf = image::RgbImage::new(mx_input as u32, my_input as u32);

        for y in 0..my_input {
            for x in 0..mx_input {
                let mut n = 0;

                let mut zxy = Complex {
                    re: y as f64 * 4.0 / (my_input - 2) as f64,
                    im: -(x as f64 * 4.0 / (mx_input + 2) as f64),
                };

                while n < iter
                // && (abs(zxy.sub(r1)) > tolerance)
                // && (abs(zxy.sub(r2)) >= tolerance)
                // && (abs(zxy.sub(r3)) >= tolerance)
                {
                    zxy = zxy.sub(zfunc(zxy) / dfunc(zxy)); // epsilon = -(zfunc(t).div(dfunc(t)));
                    n = n + 1;
                }

                if (zxy.re.add(1.0) * zxy.re.add(1.0)).add(zxy.im.mul(zxy.im)) < tolerance {
                    imgbuf.put_pixel(x as u32, y as u32, image::Rgb([255, 50, 255]));
                }

                if (zxy.re.sub(1.0) * zxy.re.sub(1.0)).add(zxy.im.mul(zxy.im)) < tolerance {
                    imgbuf.put_pixel(x as u32, y as u32, image::Rgb([100, 255, 255]));
                }

                if (zxy.re * zxy.re).add(zxy.im * zxy.im) < tolerance {
                    imgbuf.put_pixel(x as u32, y as u32, image::Rgb([255, 255, 200]));
                }

                // if abs(zxy.sub(r1)) < tolerance {
                //     imgbuf.put_pixel(x as u32, y as u32, image::Rgb([255 - x as u8, 0, 0]));
                // }
                // if abs(zxy.sub(r2)) < tolerance {
                //     imgbuf.put_pixel(x as u32, y as u32, image::Rgb([0, 255 - x as u8, 0]));
                // }
                // if abs(zxy.sub(r3)) < tolerance {
                //     imgbuf.put_pixel(x as u32, y as u32, image::Rgb([0, 0, 255 - x as u8]));
                // }
            }
        }

        imgbuf.save("fractal.png").unwrap();

        // let mx: i32 = mx_input / 2; // начало экранных координат
        // let my: i32 = my_input / 2; // начало экранных координат
        // let mut p: f64 = 0.0;
        // let mut z = Complex { re: 0.0, im: 0.0 }; // Основаная функция
        // let mut t = Complex { re: 0.0, im: 0.0 };
        // let mut d = Complex { re: 0.0, im: 0.0 }; // Производная функция
        // let bounds = (mx_input, my_input);

        // let mut pixels = vec![0; bounds.0 as usize * bounds.1 as usize];
        // let scalex = 4.0 / mx_input as f32;
        // let scaley = 4.0 / my_input as f32;

        // for y in 0..my {
        //     for x in 0..my {
        //         n = 0;
        //         let mut zt: Complex<f64> = Complex {
        //             re: x as f64 * 4.0 / mx as f64 - 2 as f64,
        //             im: -y as f64 * 4.0 / my as f64 + 2 as f64,
        //         };
        //         for i in 0..iter {
        //             if abs(zfunc(zt)) < tol {
        //                 break;
        //             }
        //             zt = zt - zfunc(zt) / dfunc(z);
        //             n = n + 1;
        //         }
        //         imgbuf.put_pixel(x as u32, y as u32, image::Luma([n as u8]));
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
        //     }
        // }
    }
}
