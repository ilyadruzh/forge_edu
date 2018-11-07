extern crate image;
extern crate num;

pub mod newtone_fractal {
    use image::png::PNGEncoder;
    use image::{ColorType, GrayImage, ImageBuffer, Luma, Pixel};
    use num::pow;
    use std::clone::Clone;
    use std::f64;
    use std::fs::File;
    use std::io::{Error, Write};
    use std::ops::{Add, Mul};
    use std::str::FromStr;

    static ITER: i64 = 50;
    static MIN: f64 = 1e-6;
    static MAX: f64 = 1e+6;

    // TODO: реализовать умножение, сложение и деление для Complex<T> и norm_sqr
    // Метод z.norm_sqr() возвращает квадрат расстояния от z до начала координат

    trait ComplexOperations {
        fn norm_sqr(&self) -> f64;
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Complex<T> {
        x: T,
        y: T,
    }

    // TODO: task1
    impl ComplexOperations for Complex<f64> {
        fn norm_sqr(&self) -> f64 {
            return 0.0;
        }
    }

    impl Mul for Complex<f64> {
        type Output = Complex<f64>;
        fn mul(self, other: Complex<f64>) -> Complex<f64> {
            Complex {
                x: self.x * other.x,
                y: self.y * other.y,
            }
        }
    }

    impl Add for Complex<f64> {
        type Output = Complex<f64>;
        fn add(self, other: Complex<f64>) -> Complex<f64> {
            Complex {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    pub fn create_img(img_w: u32, img_h: u32, iter: u64, x_0: f64, x_n: f64, y_0: f64, y_n: f64) {
        let max_iterations = iter;
        let mut n: u64 = 0; // итератор

        let mut mx: u32 = img_w / 2; // начало экранных координат
        let mut my: u32 = img_h / 2; // начало экранных координат

        let mut x_0: f64 = x_0; // диапазоны х
        let mut x_n: f64 = x_n; // диапазоны х
        let mut y_0: f64 = y_0; // диапазоны у
        let mut y_n: f64 = y_n; // диапазоны у

        let mut p: f64 = 0.0;

        let mut z = Complex { x: 0.0, y: 0.0 };
        let mut t = Complex { x: 0.0, y: 0.0 };
        let mut d = Complex { x: 0.0, y: 0.0 };

        let scalex = 4.0 / img_w as f64;
        let scaley = 4.0 / img_h as f64;

        // Create a new ImgBuf with width: imgx and height: imgy
        let mut imgbuf = GrayImage::new(img_w, img_h);

        // Iterate over the coordinates and pixels of the image
        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let cx = x as f64 * scalex - 2.0;
            let cy = y as f64 * scaley - 2.0;

            z.x = x as f64 * 0.005;
            z.y = y as f64 * 0.005;

            d = z;

            while ((pow(z.x, 2) + pow(z.y, 2)) < MAX)
                && ((pow(d.x, 2) + pow(d.y, 2)) > MIN)
                && (n < max_iterations)
            {
                t = z;
                p = pow(pow(t.x, 2) + pow(t.y, 2), 2);

                z.x = (2 as f64 / 3 as f64) * t.x + (pow(t.x, 2) - pow(t.y, 2)) / (3 as f64 * p);
                z.y = (2 as f64 / 3 as f64) * t.y * (1 as f64 - t.x / p);
                d.x = t.x.abs() - z.x.abs();
                d.y = t.y.abs() - z.y.abs();
                n = n + 1;

                *pixel = Luma([n as u8]);
            }

            // let c = Complex { x: -0.4, y: 0.6 };
            // let mut i = 0;
            // for t in 0..max_iterations {
            //     // println!("z.norm_sqr(): {}", z.norm_sqr());
            //     if z.norm_sqr() > 2.0 {
            //         break;
            //     }
            //     z = z * z + c;
            //     i = t;
            // }

            // // Create an 8bit pixel of type Luma and value i
            // // and assign in to the pixel at position (x, y)
            // *pixel = Luma([i as u8]);
        }

        // Save the image as “fractal.png”, the format is deduced from the path
        imgbuf.save("fractal.png").unwrap();
    }

    #[allow(dead_code)]
    pub fn draw(mx_input: i64, my_input: i64, x_0: f64, x_n: f64, y_0: f64, y_n: f64) {
        let mut n: i64 = 0; // итератор
        let mut mx: i64 = mx_input / 2; // начало экранных координат
        let mut my: i64 = my_input / 2; // начало экранных координат

        let mut x_0: f64 = x_0; // диапазоны х
        let mut x_n: f64 = x_n; // диапазоны х
        let mut y_0: f64 = y_0; // диапазоны у
        let mut y_n: f64 = y_n; // диапазоны у

        let mut p: f64 = 0.0;

        let mut z = Complex { x: 0.0, y: 0.0 };
        let mut t = Complex { x: 0.0, y: 0.0 };
        let mut d = Complex { x: 0.0, y: 0.0 };

        for y in -my..my {
            println!("y: {}", y);
            for x in -mx..my {
                n = 0; // счетчик итераций
                z.x = x as f64 * 0.005;
                z.y = y as f64 * 0.005;
                d = z;

                while ((pow(z.x, 2) + pow(z.y, 2)) < MAX)
                    && ((pow(d.x, 2) + pow(d.y, 2)) > MIN)
                    && (n < ITER)
                {
                    t = z;
                    p = pow(pow(t.x, 2) + pow(t.y, 2), 2);

                    z.x =
                        (2 as f64 / 3 as f64) * t.x + (pow(t.x, 2) - pow(t.y, 2)) / (3 as f64 * p);
                    z.y = (2 as f64 / 3 as f64) * t.y * (1 as f64 - t.x / p);
                    d.x = t.x.abs() - z.x.abs();
                    d.y = t.y.abs() - z.y.abs();
                    n = n + 1;
                    println!("z: {} - {}", z.x, z.y);
                    println!("t: {} - {}", t.x, t.y);
                    println!("d: {} - {}", d.x, d.y);
                    println!("p: {}", p);
                }

                // Выбираем цвет - pen.Color = Color.FromArgb(255, (n*9) % 255, 0, (n*9) % 255);
                // Рисуем прямоугольник - g.DrawRectangle(pen, mx + x, my + y, 1, 1);
                // draw_newtone_fractal(mx + x, my + y);
            }
        }
    }

    fn escape_time(c: Complex<f64>) -> Option<u32> {
        let mut z = Complex { x: 0.0, y: 0.0 };
        for i in 0..ITER {
            z = z * z + c;
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
            Some((x, y)) => Some(Complex { x, y }),
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
        let (width, height) = (lower_right.x - upper_left.x, upper_left.y - lower_right.y);
        Complex {
            x: upper_left.x + pixel.0 as f64 * width / bounds.0 as f64,
            y: upper_left.y - pixel.1 as f64 * height / bounds.1 as f64, // Почему здесь вычитание? pixel.1 увеличивается при движении вниз,
                                                                         // тогда как мнимая часть увеличивается при движении вверх.
        }
    }

    /// Рисует прямоугольную часть множества Мандельброта в буфере пикселей.
    ///
    /// Аргумент `bounds` задает ширину и высоту буфера `pixels`, в котором каждый байт
    /// представляет один полутоновый пиксель. Аргументы `upper_left` и `lower_right`
    /// определяют точки на комплексной плоскости, соответствующие левому верхнему
    /// и правому нижнему углам буфера пикселей.
    pub fn render(
        pixels: &mut [u8],
        bounds: (usize, usize),
        upper_left: Complex<f64>,
        lower_right: Complex<f64>,
    ) {
        assert!(pixels.len() == bounds.0 * bounds.1);
        for row in 0..bounds.1 {
            for column in 0..bounds.0 {
                let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
                pixels[row * bounds.0 + column] = match escape_time(point) {
                    None => 0,
                    Some(count) => 255 - count as u8,
                };
            }
        }
    }

    /// Записывает буфер `pixels`, размеры которого заданы аргументом `bounds`, в файл
    /// с именем `filename`.
    pub fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), Error> {
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

    fn point_color(x: f64) -> u64 {
        match x {
            0.0 => 000,
            1.0 => 010,
            _ => 0,
        }
    }
}
