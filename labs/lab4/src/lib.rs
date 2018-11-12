extern crate image;
extern crate num;
extern crate num_complex;

pub mod newtone_fractal {
    use image::png::PNGEncoder;
    use image::{ColorType, GrayImage, ImageBuffer, Luma, Pixel};
    use num_complex::Complex;
    use std::clone::Clone;
    use std::f64;
    use std::fs::File;
    use std::io::{Error, Write};
    use std::ops::{Add, Mul};
    use std::str::FromStr;

    ////////////////// COMPLEX /////////////////////////////

    pub fn abs(z: Complex<f64>) -> f64 {
        ((z.re * z.re) + (z.im * z.im)).sqrt()
    }

    // #[derive(Copy, Clone, Debug)]
    // pub struct Complex<T> {
    //     re: T,
    //     im: T,
    // }

    // pub fn sqr(self_: Complex<f64>) -> Complex<f64> {
    //     Complex {
    //         re: (self_.re * self_.re) - (self_.im * self_.im),
    //         im: (2 as f64 * self_.re * self_.im),
    //     }
    // }

    // pub fn sqr_norm(self_: Complex<f64>) -> Complex<f64> {
    //     Complex {
    //         re: self_.re * self_.re,
    //         im: self_.im * self_.im,
    //     }
    // }

    // // Как происходит умножение комплексных чисел
    // pub fn mul(self_: Complex<f64>, other: Complex<f64>) -> Complex<f64> {
    //     Complex {
    //         re: (self_.re * other.re) - (self_.im * other.im),
    //         im: (self_.re * other.im) + (self_.im * other.re),
    //     }
    // }

    // pub fn add(self_: Complex<f64>, other: Complex<f64>) -> Complex<f64> {
    //     Complex {
    //         re: self_.re + other.re,
    //         im: self_.im + other.im,
    //     }
    // }

    // // Как происходит вычитание комплексных чисел
    // pub fn sub(self_: Complex<f64>, other: Complex<f64>) -> Complex<f64> {
    //     // Complex {
    //     //     re: ((self_.re * other.re) + (self_.im * other.im))
    //     //         / ((other.re * other.re) + (other.im * other.im)),
    //     //     im: (-(self_.re * other.im) + (self_.im * other.re))
    //     //         / (other.re * other.re + other.im * other.im),
    //     // }

    //     Complex {
    //         re: self_.re - other.re,
    //         im: self_.im - other.im,
    //     }
    // }

    // pub fn abs(self_: Complex<f64>) -> f64 {
    //     f64::sqrt((self_.re * self_.re) + (self_.im * self_.im))
    // }

    // pub fn arg(z: Complex<f64>) -> f64 {
    //     z.im.atan2(z.re)
    // }

    // pub fn pow(z: Complex<f64>, n: i32) -> Complex<f64> {
    //     let i: i32 = 0;
    //     let mut z1: Complex<f64> = Complex { re: 1.0, im: 0.0 };
    //     for x in i..n {
    //         z1 = mul(z, z1);
    //     }
    //     return z1;
    // }

    fn escape_time(c: Complex<f64>, iter: u64) -> Option<u32> {
        let mut z = Complex { re: 0.0, im: 0.0 };
        for i in 0..iter {
            z = z.mul(z) + c;
            if z.norm_sqr() > 4.0 {
                return Some(i as u32);
            }
        }
        None
    }

    // TODO: потом сделать.
    // Разбирает строку содержащие координаты на кортеж
    pub fn parse_display_size<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
        match s.find(separator) {
            None => None,
            Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None,
            },
        }
    }

    pub fn parse_complex(s: &str) -> Option<Complex<f64>> {
        match parse_display_size(s, ',') {
            Some((re, im)) => Some(Complex { re, im }),
            None => None,
        }
    }

    /// Зная строку и столбец пикселя выходного изображения, возвращает соответствующую
    /// точку на комплексной плоскости.
    ///
    /// `bounds` - пара, определяющая ширину и высоту изображения в пикселях.
    /// `pixel` - пара (строка, столбец), определяющая конкретный пиксель изображения.
    /// Параметры `upper_left` и `lower_right` - точки на комплексной плоскости,
    /// описывающие область, покрываемую изображением.
    fn pixel_to_point(
        bounds: (usize, usize),
        pixel: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) -> Complex<f64> {
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

    /// Рисует прямоугольную часть множества Мандельброта в буфере пикселей.
    ///
    /// Аргумент `bounds` задает ширину и высоту буфера `pixels`, в котором каждый байт
    /// представляет один полутоновый пиксель. Аргументы `upper_left` и `lower_right`
    /// определяют точки на комплексной плоскости, соответствующие левому верхнему
    /// и правому нижнему углам буфера пикселей.
    fn render(
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
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

    /// Записывает буфер `pixels`, размеры которого заданы аргументом `bounds`, в файл
    /// с именем `filename`.
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

    fn point_color(x: f64) -> i64 {
        match x {
            0.0 => 000,
            1.0 => 010,
            _ => 0,
        }
    }

    ///////////////////////////// CREATE IMG /////////////////////////////

    pub fn choose_color(x: i32, y: i32, n: u32) -> (u32, u32, u32) {
        // В зависимисомти, от X, Y, N выбирается цвет
        (0, 0, 0)
    }

    pub fn create_image(x: i32, y: i32, color: (u32, u32, u32)) {}

    ///////////////////////////// NEWTOM /////////////////////////////

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
                n = 0;
                d = z;

                // Условия, при которых комплексное число устремляется в бесконечность
                // Первый параметр - функция от Z - функции комплексного числа
                // Второй параметр функция от D - производная от функции
                // Третий параметр N - счетчик

                let mut c: Complex<f64> = Complex {
                    re: mx as f64,
                    im: my as f64,
                };

                while (abs(z) < max && n < iter) {
                    z = z.mul(z).add(c);
                    n = n + 1;
                }

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

                imgbuf.put_pixel(x as u32, y as u32, image::Luma([n as u8]));
            }
        }

           imgbuf.save("fractal.png").unwrap();

        // render(
        //     &mut pixels,
        //     (bounds.0 as usize, bounds.1 as usize),
        //     Complex { re: x_0, im: y_0 },
        //     Complex { re: x_n, im: y_n },
        // );

        // write_image(
        //     "fractal.png",
        //     &pixels,
        //     (bounds.0 as usize, bounds.1 as usize),
        // )
        // .expect("ошибка при записи PNG-файла");
    }

}
