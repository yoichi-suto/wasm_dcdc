use wasm_bindgen::prelude::*;

// 一次式の計算 (ax + b)
#[wasm_bindgen]
pub fn linear_function(a: f64, b: f64, x: f64) -> f64 {
    a * x + b
}

// duty
#[wasm_bindgen]
pub fn duty_function(vin: f64, vout: f64) -> f64 {
    (vout / vin) * 100.0
}