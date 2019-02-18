extern crate wasm_bindgen;

pub mod complex_f32 {
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen]
    #[derive(Copy, Clone, Debug)]
    pub struct Complexf32 {
        pub re: f32,
        pub im: f32,
    }

    #[wasm_bindgen]
    pub fn mul(self_: Complexf32, other: Complexf32) -> Complexf32 {
        Complexf32 {
            re: (self_.re.clone() * other.re.clone()) - (self_.im.clone() * other.im.clone()),
            im: (self_.re * other.im) + (self_.im * other.re),
        }
    }

    #[wasm_bindgen]
    pub fn add(self_: Complexf32, other: Complexf32) -> Complexf32 {
        Complexf32 {
            re: self_.re + other.re,
            im: self_.im + other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub(self_: Complexf32, other: Complexf32) -> Complexf32 {
        Complexf32 {
            re: self_.re - other.re,
            im: self_.im - other.im,
        }
    }

    #[wasm_bindgen]
    pub fn sub_f32(self_: Complexf32, other: f32) -> Complexf32 {
        Complexf32 {
            re: self_.re - other,
            im: self_.im,
        }
    }

    #[wasm_bindgen]
    pub fn abs(z: Complexf32) -> f32 {
        f32::sqrt((z.re * z.re) + (z.im * z.im))
    }

    #[wasm_bindgen]
    pub fn arg(z: Complexf32) -> f32 {
        z.im.atan2(z.re)
    }

    #[wasm_bindgen]
    pub fn scale(z: Complexf32, n: f32) -> Complexf32 {
        Complexf32 {
            re: z.re.clone() * n.clone(),
            im: z.im.clone() * n,
        }
    }

    #[wasm_bindgen]
    pub fn norm_sqr(self_: Complexf32) -> f32 {
        self_.re.clone() * self_.re.clone() + self_.im.clone() * self_.im.clone()
    }

    #[wasm_bindgen]
    pub fn div(self_: Complexf32, other: Complexf32) -> Complexf32 {
        let norm_sqr = norm_sqr(other);

        Complexf32 {
            re: (self_.re.clone() * other.re.clone() + self_.im.clone() * other.im.clone())
                / norm_sqr.clone(),
            im: (self_.im * other.re - self_.re * other.im) / norm_sqr.clone(),
        }
    }
}