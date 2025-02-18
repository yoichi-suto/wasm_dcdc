use wasm_bindgen::prelude::*;
use polars::prelude::*;
use serde::{Serialize, Deserialize};
use serde_json::json;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn wasm_df(vin: f64, vout: f64, fsw: f64) -> JsValue {
    // x_value を 1 ~ 100 までの範囲で生成
    let x_values: Vec<f64> = (1..=100).map(|x| x as f64).collect();
    
    // y_value を計算
    let y_values: Vec<f64> = x_values.iter().map(|&x| x / fsw * (vin / vout)).collect();

    // DataFrame を作成
    let df = df!(
        "x_value" => x_values,
        "y_value" => y_values
    ).unwrap();

    // JSON に変換
    let json = serde_json::to_string(&df).unwrap();
    
    // JavaScript で扱えるように JsValue で返す
    JsValue::from_str(&json)
}
