extern crate wasm_bindgen;

pub mod complex_f64 {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Copy, Clone, Debug)]
    pub struct Complexf64 {
        pub re: f64,
        pub im: f64,
    }

    #[wasm_bindgen]
    pub fn mul_f64(self_: Complexf64, other: Complexf64) -> Complexf64 {
        Complexf64 {
            re: (self_.re.clone() * other.re.clone()) - (self_.im.clone() * other.im.clone()),
            im: (self_.re * other.im) + (self_.im * other.re),
        }
    }

    #[wasm_bindgen]
    pub fn add_f64(self_: Complexf64, other: Complexf64) -> Complexf64 {
        Complexf64 {
            re: self_.re + other.re,
            im: self_.im + other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub_from_f64(self_: Complexf64, other: Complexf64) -> Complexf64 {
        Complexf64 {
            re: self_.re - other.re,
            im: self_.im - other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub_f64(self_: Complexf64, other: f64) -> Complexf64 {
        Complexf64 {
            re: self_.re - other,
            im: self_.im,
        }
    }

    #[wasm_bindgen]
    pub fn abs_f64(z: Complexf64) -> f64 {
        f64::sqrt((z.re * z.re) + (z.im * z.im))
    }

    #[wasm_bindgen]
    pub fn arg_64(z: Complexf64) -> f64 {
        z.im.atan2(z.re)
    }

    #[wasm_bindgen]
    pub fn scale_f64(z: Complexf64, n: f64) -> Complexf64 {
        Complexf64 {
            re: z.re.clone() * n.clone(),
            im: z.im.clone() * n,
        }
    }

    #[wasm_bindgen]
    pub fn norm_sqr_f64(self_: Complexf64) -> f64 {
        self_.re.clone() * self_.re.clone() + self_.im.clone() * self_.im.clone()
    }

    #[wasm_bindgen]
    pub fn div_f64(self_: Complexf64, other: Complexf64) -> Complexf64 {
        let norm_sqr = norm_sqr_f64(other);

        Complexf64 {
            re: (self_.re.clone() * other.re.clone() + self_.im.clone() * other.im.clone())
                / norm_sqr.clone(),
            im: (self_.im * other.re - self_.re * other.im) / norm_sqr.clone(),
        }
    }
}